<script setup>
import { ref } from "vue";

const tag = ref("NoTag");

defineExpose({ tag });
</script>

<template>
    <select v-model="sortingMethod">
        <option value="NoTag">No tag</option>
        <option value="Work">Work</option>
        <option value="Study">Study</option>
        <option value="Hobby">Hobby</option>
        <option value="Housework">Housework</option>
        <option value="Shopping">Shopping</option>
    </select>
</template>
