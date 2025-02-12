<template>
    <n-flex justify="space-between">
        <n-button text :disabled="!running">
            <n-icon>
                <ArrowBack />
            </n-icon>
            上一步
        </n-button>

        <n-button text>
            <n-icon>
                <NavigateOutline />
            </n-icon>
            新导航
        </n-button>
        <n-dropdown trigger="hover" :options="options" @select="handleSelect">
            
            <n-button text>
                <n-icon>
                    <Add />
                </n-icon>
                更多
            </n-button>
        </n-dropdown>
        <n-button text :disabled="!running">
            下一步
            <n-icon>
                <ArrowForward />
            </n-icon>
        </n-button>
    </n-flex>
</template>

<script lang="ts">
import { NFlex, NButton, NIcon, NDropdown } from 'naive-ui';
import { ArrowBack, ArrowForward, NavigateOutline, Add } from '@vicons/ionicons5';
import { listen } from '@tauri-apps/api/event';



export default {
    components: {
        NFlex, NButton, ArrowBack, NIcon, ArrowForward, NavigateOutline, Add, NDropdown
    },
    data() {
        return {
            options: [{
                label: "绘制地图",
                key: 1
            },{
                label: "导入地图",
                key: 2
            },{
                label: "关于",
                key: 3
            },
            ],
            running: false
        };
    },
    methods: {
        async setBeginListener() {
            await listen<number>('route-move', () => {
                this.running = true;
            });
        },
        handleSelect(key: number) {

        }
    },
    mounted() {
        this.setBeginListener();
    },
};
</script>

<style scoped></style>