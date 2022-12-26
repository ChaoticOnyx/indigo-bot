import { defineConfig } from 'vite'

export default defineConfig({
    build: {
        manifest: true,
        rollupOptions: {
            input: [
                'base.css',
                'main.js'
            ]
        },
        outDir: '../public',
        emptyOutDir: true
    }
})