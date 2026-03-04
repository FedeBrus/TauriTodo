<script setup>
import { ref } from "vue";

const tag = ref("NoTag");

defineExpose({ tag });
</script>

<template>
    <select v-model="sortingMethod">
        <option value="NoTag">No tag</option>
        <option value="Work">Work</option>
        <option value="Study">Work</option>
        <option value="Hobby">Work</option>
        <option value="Housework">Work</option>
        <option value="Shopping">Work</option>
    </select>
</template>
