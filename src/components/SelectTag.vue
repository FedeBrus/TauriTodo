<script setup>
import { ref } from "vue";

const tag = ref("NoTag");

defineExpose({ tag });
</script>

<template>
    <select v-model="sortingMethod">
        <option value="NoTag">No tag</option>
        <option value="Work">Work</option>
    </select>
</template>
