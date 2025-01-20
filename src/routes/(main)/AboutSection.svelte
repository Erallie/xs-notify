<script lang="ts">
    let { children, title } = $props();
</script>

<div class="collapse join-item collapse-plus bg-base-300">
    <input type="radio" name="about" />
    <h2 class="collapse-title text-center text-2xl font-bold">{title}</h2>
    <div class="collapse-content space-y-6 text-pretty">
        {@render children()}
    </div>
</div>
