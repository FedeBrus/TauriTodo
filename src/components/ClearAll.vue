<script setup>
    import { invoke } from '@tauri-apps/api/core';
</script>

<template>
    <button 
        @click="async () => {
            await invoke('delete_all');
        }"
    >
        Clear All
    </button>
</template>