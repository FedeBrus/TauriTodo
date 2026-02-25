<script setup>
    import { ref, onMounted, onUnmounted } from 'vue';

    const props = defineProps(['text, expiration']);
    const textInput = ref(null);
    const dateInput = ref(null);
    const text = ref("");
    const expiration = ref(new Date().toISOString().slice(0, 10));
    const taskContainer = ref(null);

    const emit = defineEmits(['lose-focus', 'save-task']);

    onMounted(() => {
        textInput.value.focus();
        window.addEventListener('mousedown', handleClickOutside);
    });

    onUnmounted(() => {
        window.removeEventListener('mousedown', handleClickOutside);
    });

    function handleClickOutside(event) {
        if (taskContainer.value.contains(event.target)) return;
        emit('lose-focus');
    }

    function checkForSubmit() {
        if (text.value == "" || !expiration.value || expiration.value < new Date().toISOString().slice(0, 10)) {
            emit('lose-focus');
        } else {
            emit('save-task', text.value, expiration.value);
        }
    }

    defineExpose({ focus });
</script>

<template>
    <li 
        ref="taskContainer"
        class="task-list-item"
    >
        <input
            ref="textInput" 
            v-model="text" 
            type="text" 
        >
        <label class="due">
            Due to:
            <input 
                ref="dateInput"
                v-model="expiration"
                type="date"
                @change="() => { dateInput.focus(); }"
            >
        </label>
        <button
            class="confirm" 
            @click="checkForSubmit" 
        >
            <img src="../assets/confirm.svg" alt="confirm">
        </button>
    </li>
</template>