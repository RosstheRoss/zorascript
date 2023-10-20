<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, LightSwitch, initializeStores } from '@skeletonlabs/skeleton';

	import { pwaInfo } from 'virtual:pwa-info';

	$: webManifest = pwaInfo ? pwaInfo.webManifest.linkTag : '';

	initializeStores();
</script>

<svelte:head>
  <!-- eslint-disable-next-line svelte/no-at-html-tags VitePWA can probably be trusted -->
  {@html webManifest}
</svelte:head>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
			<svelte:fragment slot="lead">
				<LightSwitch />
			</svelte:fragment>
			<a href="#">Zelda: Oracle Password Generator</a>
			<svelte:fragment slot="trail">
				<a href="#">TODO: SOURCE CODE</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
