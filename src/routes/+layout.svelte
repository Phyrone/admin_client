<script lang="ts">
    import '$styles/main.scss'
</script>
<div class="flex flex-col h-full bg-green-100 overflow-clip">
    <div class="h-10 flex-none">
        Head
    </div>
    <div class="flex-auto px-4 mx-4 shadow-2xl flex flex-col">
        <slot/>
    </div>
    <div class="flex-none">
        Foot
    </div>

</div>
