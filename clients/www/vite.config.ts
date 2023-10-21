import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';

export default defineConfig({
	build: {
		sourcemap: true
	},
	css: {
		devSourcemap: false
	},
	plugins: [
		sveltekit(),
		SvelteKitPWA({
			devOptions: {
				enabled: true,
				type: 'module'
			},
			manifest: {
				name: 'Oracle Client',
				short_name: 'Oracle Client',
				theme_color: '#3EBCC5',
				description: "A web-based client for the Oracle of Ages/Seasons password system",
				display_override: ["window-controls-overlay", "standalone", "minimal-ui"],
				display: "fullscreen",
				icons: [
					{
						src: "img/pwa-192x192.png",
						sizes: "192x192",
						type: "image/png",
					}
				],
				id: "oracle-client",
			},
			registerType: 'autoUpdate',
			workbox: {
				globPatterns: ['client/**/*.{js,css,html,svg,png}']
			}
		}),
		purgeCss()
	]
});
