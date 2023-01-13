use crate::prelude::*;
use app_macros::global;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
struct OsUserAgentRegex {
    pub regex: Regex,
    pub display: String,
}

impl OsUserAgentRegex {
    pub fn new(regex: &str, display: impl Into<String>) -> Self {
        let regex = Regex::new(regex).unwrap();
        let display = display.into();

        Self { regex, display }
    }
}

#[derive(Debug, Clone)]
pub struct BrowserUserAgentRegex {
    pub regex: Regex,
    pub display: String,
}

impl BrowserUserAgentRegex {
    pub fn new(regex: &str, display: impl Into<String>) -> Self {
        let regex = Regex::new(regex).unwrap();
        let display = display.into();

        Self { regex, display }
    }
}

#[derive(Debug, Clone)]
#[global(clone, set)]
pub struct UserAgentParser {
    os_regexs: Vec<OsUserAgentRegex>,
    browser_regexs: Vec<BrowserUserAgentRegex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgent {
    pub os: Option<String>,
    pub browser: Option<String>,
}

impl UserAgentParser {
    #[instrument]
    pub fn parse(&self, user_agent: &str) -> UserAgent {
        trace!("parse");

        let mut os = None;

        for os_regex in &self.os_regexs {
            if os_regex.regex.is_match(user_agent) {
                let mut display = os_regex.display.clone();
                let captures = os_regex.regex.captures(user_agent).unwrap();

                let mut idx = 1;
                for capture in captures.iter().skip(1) {
                    let Some(capture) = capture else {
                        continue;
                    };

                    display = display.replace(&format!("${idx}"), capture.as_str());
                    idx += 1;
                }

                os = Some(display);
                break;
            }
        }

        let mut browser = None;

        for browser_regex in &self.browser_regexs {
            if browser_regex.regex.is_match(user_agent) {
                let mut display = browser_regex.display.clone();
                let captures = browser_regex.regex.captures(user_agent).unwrap();

                let mut idx = 1;
                for capture in captures.iter().skip(1) {
                    let Some(capture) = capture else {
                        continue;
                    };

                    display = display.replace(&format!("${idx}"), capture.as_str());
                    idx += 1;
                }

                browser = Some(display);
                break;
            }
        }

        UserAgent { os, browser }
    }
}

impl Default for UserAgentParser {
    fn default() -> Self {
        let os_regexs = vec![
            OsUserAgentRegex::new(r"(Windows 10)", "Windows"),
            OsUserAgentRegex::new(r"(Windows (?:NT 5\.2|NT 5\.1))", "Windows XP"),
            OsUserAgentRegex::new(r"(Win(?:dows NT |32NT/)6\.1)", "Windows 7"),
            OsUserAgentRegex::new(r"(Win(?:dows NT |32NT/)6\.0)", " Windows Vista"),
            OsUserAgentRegex::new(r"(Win(?:dows NT |32NT/)6\.2)", "Windows 8"),
            OsUserAgentRegex::new(r"(Win(?:dows NT |32NT/)6\.3)", "Windows 8.1"),
            OsUserAgentRegex::new(r"(Win(?:dows NT |32NT/)6\.4)", "Windows 10"),
            OsUserAgentRegex::new(r"(Windows NT 10\.0)", "Windows 10"),
            OsUserAgentRegex::new(
                r"Win(?:dows)? ?(95|98|3.1|NT|ME|2000|XP|Vista|7|CE)",
                "Windows $1",
            ),
            OsUserAgentRegex::new(
                r"((?:Mac[ +]?|; )OS[ +]X)[\s+/](?:(\d+)[_.](\d+)(?:[_.](\d+)|)|Mach-O)",
                "Mac OS X",
            ),
            OsUserAgentRegex::new(r"Mac OS X\s.{1,50}\s(\d+).(\d+).(\d+)", "Mac OS X $1.$2.$3"),
            OsUserAgentRegex::new(
                r" (Dar)(win)/(9).(\d+).{0,100}\((?:i386|x86_64|Power Macintosh)\)",
                "Mac OS X 10.5",
            ),
            OsUserAgentRegex::new(
                r" (Dar)(win)/(10).(\d+).{0,100}\((?:i386|x86_64)\)",
                "Mac OS X 10.6",
            ),
            OsUserAgentRegex::new(
                r" (Dar)(win)/(11).(\d+).{0,100}\((?:i386|x86_64)\)",
                "Mac OS X 10.7",
            ),
            OsUserAgentRegex::new(
                r" (Dar)(win)/(12).(\d+).{0,100}\((?:i386|x86_64)\)",
                "Mac OS X 10.8",
            ),
            OsUserAgentRegex::new(
                r" (Dar)(win)/(13).(\d+).{0,100}\((?:i386|x86_64)\)",
                "Mac OS X 10.9",
            ),
            OsUserAgentRegex::new(
                r"(CPU[ +]OS|iPhone[ +]OS|CPU[ +]iPhone|CPU IPhone OS|CPU iPad OS)[ +]+(\d+)[_\.](\d+)(?:[_\.](\d+)|)",
                "iOS",
            ),
            OsUserAgentRegex::new(r"(iPhone|iPad|iPod); Opera", "iOS"),
            OsUserAgentRegex::new(
                r"(iPhone|iPad|iPod).{0,100}Mac OS X.{0,100}Version/(\d+)\.(\d+)",
                "iOS",
            ),
            OsUserAgentRegex::new(r"(CrOS) [a-z0-9_]+ (\d+)\.(\d+)(?:\.(\d+)|)", "Chrome OS"),
            OsUserAgentRegex::new(r"([Dd]ebian)", "Debian"),
            OsUserAgentRegex::new(r"(Linux Mint)(?:/(\d+)|)", "Linux Mint $1"),
            OsUserAgentRegex::new(
                r"(Mandriva)(?: Linux|)/(?:[\d.-]+m[a-z]{2}(\d+).(\d)|)",
                "Linux Mandriva $1",
            ),
            OsUserAgentRegex::new(
                r"(Fedora|Red Hat|PCLinuxOS|Puppy|Ubuntu|Kindle|Bada|Sailfish|Lubuntu|BackTrack|Slackware|(?:Free|Open|Net|\b)BSD)[/ ](\d+)\.(\d+)(?:\.(\d+)|)(?:\.(\d+)|)",
                "Linux $1",
            ),
            OsUserAgentRegex::new(
                r"(Linux)[ /](\d+)\.(\d+)(?:\.(\d+)|).{0,100}gentoo",
                "Linux Gentoo $1",
            ),
            OsUserAgentRegex::new(r"(Linux)(?:[ /](\d+)\.(\d+)(?:\.(\d+)|)|)", "Linux $1"),
            OsUserAgentRegex::new(
                r"(Android)[ \-/](\d+)(?:\.(\d+)|)(?:[.\-]([a-z0-9]+)|)",
                "Android $1",
            ),
        ];

        let browser_regexs = vec![
            BrowserUserAgentRegex::new(r"(PaleMoon)/(\d+)\.(\d+)(?:\.(\d+)|)", "Pale Moon"),
            BrowserUserAgentRegex::new(
                r"(Fennec)/(\d+)\.(\d+)\.?([ab]?\d+[a-z]*)",
                "Firefox Mobile",
            ),
            BrowserUserAgentRegex::new(r"(Fennec)/(\d+)\.(\d+)(pre)", "Firefox Mobile"),
            BrowserUserAgentRegex::new(r"(Fennec)/(\d+)\.(\d+)", "Firefox Mobile"),
            BrowserUserAgentRegex::new(
                r"(?:Mobile|Tablet);.{0,200}(Firefox)/(\d+)\.(\d+)",
                "Firefox Mobile",
            ),
            BrowserUserAgentRegex::new(
                r"(Namoroka|Shiretoko|Minefield)/(\d+)\.(\d+)\.(\d+(?:pre|))",
                "Firefox ($1)",
            ),
            BrowserUserAgentRegex::new(
                r"(?:Mobile Safari).{1,300}(OPR)/(\d+)\.(\d+)\.(\d+)",
                "Opera Mobile",
            ),
            BrowserUserAgentRegex::new(r"(?:Chrome).{1,300}(OPR)/(\d+)\.(\d+)\.(\d+)", "Opera"),
            BrowserUserAgentRegex::new(r"(OPiOS)/(\d+).(\d+).(\d+)", "Opera Mini"),
            BrowserUserAgentRegex::new(r"Windows Phone .{0,200}(Edge)/(\d+)\.(\d+)", "Edge Mobile"),
            BrowserUserAgentRegex::new(
                r"(EdgiOS|EdgA)/(\d+)\.(\d+)\.(\d+)(?:\.(\d+)|)",
                "Edge Mobile",
            ),
            BrowserUserAgentRegex::new(r"Mobile.{0,200}(DuckDuckGo)/(\d+)", "DuckDuckGo Mobile"),
            BrowserUserAgentRegex::new(r"(CrMo)/(\d+)\.(\d+)\.(\d+)\.(\d+)", "Chrome Mobile"),
            BrowserUserAgentRegex::new(r"(CriOS)/(\d+)\.(\d+)\.(\d+)\.(\d+)", "Chrome Mobile iOS"),
            BrowserUserAgentRegex::new(
                r"(Chrome)/(\d+)\.(\d+)\.(\d+)\.(\d+) Mobile(?:[ /]|$)",
                "Chrome Mobile",
            ),
            BrowserUserAgentRegex::new(
                r" Mobile .{1,300}(Chrome)/(\d+)\.(\d+)\.(\d+)\.(\d+)",
                "Chrome Mobile",
            ),
            BrowserUserAgentRegex::new(r"(YaBrowser)/(\d+)\.(\d+)\.(\d+)", "Yandex Browser"),
            BrowserUserAgentRegex::new(
                r"(Edge?)/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)(?:\.(\d+)|)",
                "Edge",
            ),
            BrowserUserAgentRegex::new(r"(brave)/(\d+)\.(\d+)\.(\d+) Chrome", "Brave"),
            BrowserUserAgentRegex::new(
                r"(Chromium|Chrome)/(\d+)\.(\d+)(?:\.(\d+)|)(?:\.(\d+)|)",
                "Chrome",
            ),
        ];

        Self {
            os_regexs,
            browser_regexs,
        }
    }
}
