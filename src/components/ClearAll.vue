<script setup>
    import { invoke } from '@tauri-apps/api/core';
</script>

<template>
    <button 
        @click="async () => {
            await invoke('delete_all');
        }"
    >
        <img class="delete_all" src="../assets/delete_all.svg" alt="delete all">
    </button>
</template>