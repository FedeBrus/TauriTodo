<script setup>
import AddEntry from "./AddEntry.vue";
import NewEntry from "./NewEntry.vue";
import Task from "./Task.vue";
import { onMounted, nextTick, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ClearAll from "./ClearAll.vue";
import SelectSorting from "./SelectSorting.vue";
import { listen } from "@tauri-apps/api/event";

const isAdding = ref(false);
const tasks = ref([]);

const sortingSelectionInput = ref(null);

listen("list-changed", async (_) => {
    await nextTick();
    refresh();
});

function getTaskKey(task) {
    return `${Object.values(task).join("|")}`;
}

async function refresh() {
    let sm = sortingSelectionInput.value.sortingMethod;
    if (!sm) {
        sm = "Text";
    }
    await invoke("get_tasks", {
        sortingMethod: sortingSelectionInput.value.sortingMethod,
        ascending: true,
    })
        .then((task_list) => {
            console.log(task_list);
            tasks.value = task_list;
        })
        .catch((e) => console.error(e));
}

onMounted(() => {
    refresh();
});
</script>

<template>
    <div>
        <div class="main-control">
            <h1 class="main-title">Tasks:</h1>
            <SelectSorting
                ref="sortingSelectionInput"
                @sorting-method-changed="refresh"
            />
            <ClearAll />
        </div>
        <ul class="task-list">
            <li class="task-list-item">
                <div class="task">
                    <div>Task:</div>
                    <div>Due to:</div>
                    <div>Created in:</div>
                </div>
            </li>
            <Task
                v-for="task in tasks"
                :task_id="task.id"
                :task_text="task.text"
                :task_expiration="task.expiration"
                :task_date="task.date"
                :task_status="task.status"
                :key="getTaskKey(task)"
            />
            <NewEntry
                v-if="isAdding"
                ref="newEntry"
                @lose-focus="
                    () => {
                        isAdding = false;
                    }
                "
                @save-task="
                    async (text, expiration) => {
                        await invoke('add_task', {
                            text: text,
                            expiration: expiration,
                        })
                            .then((task) => console.log(task))
                            .catch((e) => console.error(e));
                        isAdding = false;
                    }
                "
            />
            <AddEntry
                @click="
                    () => {
                        isAdding = true;
                    }
                "
            />
        </ul>
    </div>
</template>
