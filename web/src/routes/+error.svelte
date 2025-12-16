<script>
	import { page } from '$app/state';
</script>

<div class="flex h-full w-full items-center justify-center">
	{page.error?.message}
</div>
