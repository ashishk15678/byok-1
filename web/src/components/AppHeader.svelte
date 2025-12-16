<script lang="ts">
	// imports
	import RadarIcon from '@icons/RadarIcon.svelte';
	import ButtonWithLink from './ButtonWithLink.svelte';

	let isHoveringHeader = $state(false);
	let { children } = $props();
</script>

<main class="flex h-dvh w-full flex-col">
	<header
		class="flex h-1/18 w-full items-center justify-between border-b border-border bg-secondary px-4 text-secondary-foreground"
	>
		<div
			aria-label="Application Header"
			role="banner"
			class="group flex items-center gap-x-2 text-xl font-extrabold"
			onmouseenter={() => (isHoveringHeader = true)}
			onmouseleave={() => (isHoveringHeader = false)}
		>
			<RadarIcon class="" isHovered={isHoveringHeader} />
			<span class="">Zed</span>
		</div>
		<div>
			<ButtonWithLink variant="primary" href="/login" className=" py-4 ">Get started</ButtonWithLink
			>
			<ButtonWithLink variant="link" href="/resources" className=" py-4  ">Resources</ButtonWithLink
			>
			<ButtonWithLink variant="link" href="/pricing" className=" py-4 ">Pricing</ButtonWithLink>
		</div>
		<div class="flex items-center gap-x-1 text-sm">
			<ButtonWithLink variant="link" href="/login">Login</ButtonWithLink>
			<ButtonWithLink variant="link" href="/register">Register</ButtonWithLink>
		</div>
	</header>
	<div class="h-full w-full bg-background">
		{@render children()}
	</div>
</main>
