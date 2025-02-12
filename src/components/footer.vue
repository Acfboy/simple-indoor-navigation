<template>
    <n-flex justify="space-between">
        <n-button text :disabled="!running">
            <n-icon>
                <ArrowBack />
            </n-icon>
            上一步
        </n-button>
        <n-button text @click="newNav">
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
    <n-modal v-model:show="newNavModal">
        <n-card style="width: 90%" title="新导航" :bordered="false" size="huge" role="dialog" aria-modal="true">

            <n-space vertical>
                你的位置
                <n-select v-model:value="cur" filterable placeholder="你的位置" :options="landmarks" />
                目的地
                <n-select v-model:value="dest" filterable placeholder="目的地" :options="landmarks" />

                <n-button type="success" @click="createNewNav">
                    出发
                </n-button>
            </n-space>
                <template #action>
                    请移动到最近的参照物（如教室）附近，选择你的位置和目的地，开始导航。
                    参考前进方向由磁力计确定，可能有较大误差，仅供参考。
                    请注意观察路口参照物，并跟随文字提示。
                </template>
        </n-card>
    </n-modal>
    <n-modal v-model:show="showInfo">
        <n-card style="width: 90%" title="新导航" :bordered="false" size="huge" role="dialog" aria-modal="true">

            <n-space vertical>
                你的位置
                <n-select v-model:value="cur" filterable placeholder="你的位置" :options="landmarks" />
                目的地
                <n-select v-model:value="dest" filterable placeholder="目的地" :options="landmarks" />

                <n-button type="success" @click="createNewNav">
                    出发
                </n-button>
            </n-space>
                <template #action>
                    请移动到最近的参照物（如教室）附近，选择你的位置和目的地，开始导航。
                    参考前进方向由磁力计确定，可能有较大误差，仅供参考。
                    请注意观察路口参照物，并跟随文字提示。
                </template>
        </n-card>
    </n-modal>
</template>

<script lang="ts">
import { NFlex, NButton, NIcon, NDropdown, NModal, NCard, NSpace, NSelect } from 'naive-ui';
import { ArrowBack, ArrowForward, NavigateOutline, Add } from '@vicons/ionicons5';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import themeLight from 'naive-ui/es/float-button-group/styles/light';

type landmarkItem = {
    label: string,
    value: string
}


export default {
    components: {
        NFlex, NButton, ArrowBack, NIcon, ArrowForward, NavigateOutline, Add, NDropdown,
        NModal, NCard, NSpace, NSelect
    },
    data() {
        return {
            options: [{
                label: "绘制地图",
                key: "paint"
            }, {
                label: "地图管理",
                key: "mapmanager"
            }, {
                label: "关于",
                key: "info"
            },
            ],
            running: false,
            newNavModal: false,
            landmarks: [] as landmarkItem[],
            cur: "",
            dest: "",
            showInfo: false,
        };
    },
    methods: {
        async setBeginListener() {
            await listen<number>('route-move', () => {
                this.running = true;
            });
        },
        async newNav() {
            // const resp: landmarkItem[] = await invoke("get_landmark_list");
            // this.landmarks = resp;
            this.newNavModal = true;
        },
        handleSelect(key: string) {
            this.$emit('switch', key)
        },
        async createNewNav() {
            // await invoke("get_landmark_list", {
            //     cur: this.cur,
            //     dest: this.dest
            // });
            this.newNavModal = false;
        }
    },
    mounted() {
        this.setBeginListener();
    },
};
</script>

<style scoped></style>