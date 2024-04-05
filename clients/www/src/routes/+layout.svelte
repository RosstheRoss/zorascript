<script lang="ts">
	import { base } from '$app/paths';
	import '../app.postcss';
	import { AppShell, AppBar, LightSwitch, initializeStores, Toast } from '@skeletonlabs/skeleton';

	import { pwaInfo } from 'virtual:pwa-info';

	$: webManifest = pwaInfo ? pwaInfo.webManifest.linkTag : '';

	initializeStores();
</script>

<svelte:head>
	<!-- eslint-disable-next-line svelte/no-at-html-tags VitePWA can probably be trusted -->
	{@html webManifest}
</svelte:head>

<Toast />

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
			<svelte:fragment slot="lead">
				<LightSwitch />
			</svelte:fragment>
			<a href="{base}/">Zelda: Oracle Password Generator</a>
			<svelte:fragment slot="trail">
				<a href="https://github.com/RosstheRoss/zorascript">TODO: SOURCE CODE</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>

{#await import('$lib/components/ReloadPrompt.svelte') then { default: ReloadPrompt }}
	<ReloadPrompt />
{/await}
