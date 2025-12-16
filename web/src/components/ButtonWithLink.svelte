<script lang="ts">
	import BeautifulBorderButton from './BeautifulBorderButton.svelte';

	export let href: string = '#';
	export let variant: 'primary' | 'secondary' | 'link' = 'primary';
	export let className: string = '';
	$: finalClassName = `${className} button-link`;
</script>

<a {href} class={finalClassName} {...$$restProps}>
	<BeautifulBorderButton {variant}>
		<slot />
	</BeautifulBorderButton>
</a>
