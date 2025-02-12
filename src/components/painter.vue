<template>
    <n-layout>
        <n-layout-content :style="`height: ${screenHeight * 0.9}px`">
            <!-- <n-flex justify="center">
            </n-flex> -->
            <n-tabs type="line" animated style="padding: 0 20px 0 20px; height: 100%">
                <n-tab-pane name="add" tab="添加">
                    <n-text type="info">
                        {{ '前方方位角: ' + orien }}
                    </n-text>
                    <n-flex verticle>
                        <n-card title="添加节点" :style="`height: ${screenHeight * 0.4}px`">

                            <!-- <n-form :model="curInterMark">
                                <n-form-item label="节点名称" path="name"> -->
                            节点名称
                            <n-input v-model:value="curInterMark.name" placeholder="输入名称" />
                            <!-- </n-form-item> -->
                            <!-- <n-form-item label="补充名称" path="detail"> -->
                            补充名称
                            <n-input v-model:value="curInterMark.detail" placeholder="若节点名称相同，用补充名称区分。" />
                            <!-- </n-form-item> -->
                            <n-checkbox v-model:checked="isCurInterElevator">
                                这是一个电梯节点
                            </n-checkbox>
                            <div v-if="isCurInterElevator" >
                            楼层
                            <n-input v-model:value="curFloor" placeholder="输入现在楼层"/>
                        </div>
                            <!-- </n-form> -->
                        </n-card>
                        <n-card title="添加走廊" :style="`height: ${screenHeight * 0.4}px`">
                        </n-card>
                    </n-flex>
                </n-tab-pane>
                <n-tab-pane name="del" tab="删除">
                    Hey Jude
                </n-tab-pane>
            </n-tabs>
        </n-layout-content>
        <n-layout-footer bordered>
            <n-flex justify="center">
                <n-button @click="handleBack" text>
                    <n-icon>
                        <ArrowBack />
                    </n-icon>
                    返回
                </n-button>
            </n-flex>
        </n-layout-footer>
    </n-layout>
</template>

<script lang="ts">
import {
    NCard, NLayout, NLayoutContent, NLayoutFooter, NFlex, NIcon, NText, NLayoutHeader, NTabs, NTabPane, NForm,
    NFormItem, NInput, NCheckbox
} from 'naive-ui';
import { ArrowBack, ArrowForward, NavigateOutline, Add } from '@vicons/ionicons5';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

type Corridor = {
    startPoint: Mark,
    endPoint: Mark,
    innerPoints: Mark[]
}

type Mark = {
    name: string,
    detail: string
}

type Branch = {
    direction: number,
    mark: Mark
}

type Intersection = {
    type: "normal" | "elevator",
    floorLevel: number
    branches: Branch[]
}

type OriginalMap = {
    nodes: Intersection[],
    edges: Corridor[]
}

export default {
    components: {
        NCard, NLayout, NLayoutContent, NLayoutFooter, NFlex, NIcon,
        ArrowBack, NText, NLayoutHeader, NTabs, NTabPane, NForm, NFormItem,
        NInput, NCheckbox
    },
    props: ['screenHeight'],
    data() {
        return {
            orien: 0,
            map: {
                nodes: [],
                edges: []
            } as OriginalMap,
            curInterMark: {
                name: "",
                detail: ""
            } as Mark,
            isCurInterElevator: false,
            curFloor: ""
        }
    },
    methods: {
        handleBack() {
            this.$emit('switch', 'main')
        }
    },
    mounted() {
        setInterval(async () => {
            type OrientationData = {
                orientation: number;
            };
            const resp: OrientationData = await invoke("plugin:mobilesensors|get_orientation");
            this.orien = Math.floor(resp.orientation);
        }, 10);
    },
};
</script>

<style scoped>
.n-layout-footer {
    padding: 24px;
    background-color: white;
}
</style>