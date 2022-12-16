import { defineConfig } from 'vite'

export default defineConfig({
    build: {
        manifest: true,
        rollupOptions: {
            input: [
                'base.css'
            ]
        },
        outDir: '../public',
        emptyOutDir: true
    }
})