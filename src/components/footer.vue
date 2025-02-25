<template>
    <n-flex justify="space-between">
        <n-button text :disabled="!running" @click="prevStep">
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
        <n-button text :disabled="!running" @click="nextStep">
            下一步
            <n-icon>
                <ArrowForward />
            </n-icon>
        </n-button>
    </n-flex>
    <n-modal v-model:show="newNavModal">
        <n-card style="width: 90%" title="新导航" :bordered="false" size="huge" role="dialog" aria-modal="true">
            <n-space vertical>
                选择地图
                <n-select @update:value="updateLandMarks" v-model:value="selectedMap" filterable placeholder="地图"
                    :options="maps" />
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
                地图方向由磁力计确定，可能有较大误差，仅供参考。
            </template>
        </n-card>
    </n-modal>
</template>

<script lang="ts">
import { NFlex, NButton, NIcon, NDropdown, NModal, NCard, NSpace, NSelect } from 'naive-ui';
import { ArrowBack, ArrowForward, NavigateOutline, Add } from '@vicons/ionicons5';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { BaseDirectory, exists, mkdir, readDir, readTextFile } from '@tauri-apps/plugin-fs';

type landmarkItem = {
    label: string,
    value: string
}

type selectItem = {
    label: string,
    value: string
}


export default {
    components: {
        NFlex, NButton, ArrowBack, NIcon, ArrowForward, NavigateOutline, Add, NDropdown,
        NModal, NCard, NSpace, NSelect
    },
    props: ['displayWdith', 'displayHeight'],
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
            maps: [] as selectItem[],
            curMap: "",
            running: false,
            newNavModal: false,
            landmarks: [] as landmarkItem[],
            cur: "",
            dest: "",
            showInfo: false,
            selectedMap: "",
            mapObj: { imgs: [], map: {}, scales: {} },
        };
    },
    methods: {
        async updateLandMarks(value: string) {
            const dirExists = await exists('maps', {
                baseDir: BaseDirectory.AppData
            });
            if (!dirExists) {
                await mkdir('maps', {
                    baseDir: BaseDirectory.AppData,
                });
            }
            const content = await readTextFile("maps/" + value + '.json', {
                baseDir: BaseDirectory.AppData
            });
            const mapObj = JSON.parse(content);
            this.mapObj = mapObj;
            let nameList = [];
            for (const c of mapObj.map.nodes)
                nameList.push(c);
            this.landmarks = nameList.map((s) => {
                return {
                    label: s.name,
                    value: s.index
                }
            });
        },
        async prevStep() {
            try {
                await invoke('prev_step');
            }
            catch {

            }
        },
        async nextStep() {
            try {
                await invoke('next_step');
            }
            catch {

            }
        },
        async setBeginListener() {
            await listen<number>('begin', () => {
                this.running = true;
            });
        },
        async newNav() {
            this.maps = [];
            let mapList = [];
            try {
                const entries = await readDir("maps", {
                    baseDir: BaseDirectory.AppData
                });
                for (const entry of entries) {
                    mapList.push(entry.name.slice(0, -5))
                }
                this.maps = mapList.map((s) => {
                    return {
                        label: s,
                        value: s
                    }
                });
                this.newNavModal = true;
            }
            catch (err) {
                alert(err);
            }
        },
        handleSelect(key: string) {
            this.$emit('switch', key)
        },
        async createNewNav() {
            invoke("create_new_nav", {
                from: this.cur,
                to: this.dest,
                map: this.mapObj.map,
                imgs: this.mapObj.imgs,
                scale: this.mapObj.scales,
                screen: {x: this.$props.displayWdith, y: this.$props.displayHeight }
            })
                .then(() => this.newNavModal = false, (err) => alert(err));
        }
    },
    mounted() {
        this.setBeginListener();
    },
};
</script>

<style scoped></style>