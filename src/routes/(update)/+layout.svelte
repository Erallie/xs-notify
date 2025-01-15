<script lang="ts">
    import "../../app.pcss";

    let { children } = $props();
</script>

<main class="container">
    {@render children()}
</main>
