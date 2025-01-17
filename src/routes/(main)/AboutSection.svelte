<script lang="ts">
    let { children, title } = $props();
</script>

<div class="space-y-6 text-pretty">
    <h2 class="text-center text-2xl font-bold">{title}</h2>
    {@render children()}
</div>
