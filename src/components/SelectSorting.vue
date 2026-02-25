<script setup>
import { ref } from "vue";

const emit = defineEmits(["sorting-method-changed"]);
const sortingMethod = ref("Text");

function handleChange() {
    emit('sorting-method-changed');
}

defineExpose({ sortingMethod });
</script>

<template>
    <select
        v-model="sortingMethod"
        @change="handleChange"
    >
        <option value="Text">Text</option>
        <option value="Date">Date</option>
        <option value="Expiration">Expiration</option>
        <option value="Tag">Tag</option>
    </select>
</template>
