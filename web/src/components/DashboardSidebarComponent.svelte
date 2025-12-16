<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import NfcIcon from '@icons/NfcIcon.svelte';

	type SvelteComponentType = new (...args: any) => import('svelte').SvelteComponent;

	let navItems: { key: string; label: string; href: string; icon?: SvelteComponentType }[] = [
		{ key: 'home', label: 'Home', href: '/' },
		{ key: 'about', label: 'Dashboard', href: '/dashboard' },
		{ key: 'contact', label: 'Contact', href: '/contact' },
		{ key: 'payment', label: 'Payment', href: '/payment' }
	];
</script>

<div class="flex h-full w-full items-center justify-center py-8">
	<div class="flex h-full w-5xl max-w-7xl flex-row">
		<div class="flex h-full w-1/6 flex-col gap-y-1 border-r border-secondary px-2 py-8">
			{#each navItems as item (item.key)}
				<a
					href="{base}{item.href}"
					class=" flex flex-row gap-x-4 px-4 py-1 text-sm text-muted-foreground {base + item.href ==
					$page.url.pathname
						? 'rounded-3xl bg-secondary text-primary'
						: ''}"
				>
					<NfcIcon isHovered={true} size={18} />
					{item.label}
				</a>
			{/each}
		</div>
		<div class="h-full w-full">
			<slot />
		</div>
	</div>
</div>
