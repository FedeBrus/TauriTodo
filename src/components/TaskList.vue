<script setup>
    import AddEntry from "./AddEntry.vue";
    import NewEntry from "./NewEntry.vue";
    import Task from "./Task.vue"
    import { onMounted, nextTick, ref } from "vue";
    import { invoke } from '@tauri-apps/api/core';
    import ClearAll from "./ClearAll.vue";
    import { listen } from '@tauri-apps/api/event';

    const isAdding = ref(false);
    const tasks = ref([]);

    listen('list-changed', _ => refresh());

    function toggleAdding() {
        isAdding.value = true;
    }

    async function refresh() {
        await invoke('get_tasks')
        .then((task_list) => {
            tasks.value = task_list
        })
        .catch(e => console.error(e));
    }

    onMounted(() => {
        refresh();
    });
</script>

<template>
    <div>
        <div class="main-control">
            <h1 class="main-title">Tasks:</h1>
            <ClearAll />
        </div>
        <ul class="task-list">
            <Task 
                v-for="task in tasks" 
                :task_value="task" 
            />
            <NewEntry 
                v-if="isAdding" 
                ref="newEntry" 
                @lose-focus="() => { isAdding = false }" 
                @save-task="async (text) => { 
                    await invoke('add_task', { msg: text })
                    .then((task) => console.log(task)) 
                    .catch((e) => console.error(e))
                    isAdding = false;
                }"
            />
            <AddEntry @click="toggleAdding" />
        </ul>
    </div>
</template>