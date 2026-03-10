<script setup>
import { ref } from "vue";

const props = defineProps(["initialValue"]);

const tag = ref(props.initialValue || "NoTag");

defineExpose({ tag });
</script>

<template>
    <select v-model="tag">
        <option value="NoTag">Notag</option>
        <option value="Work">Work</option>
        <option value="Study">Study</option>
        <option value="Hobby">Hobby</option>
        <option value="Housework">Housework</option>
        <option value="Shopping">Shopping</option>
    </select>
</template>
