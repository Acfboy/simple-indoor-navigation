<template>
    <n-timeline v-if="begin.length==0">
        <n-timeline-item type="error" content="导航未开始" />
        <n-timeline-item type="info" content="按照以下步骤开始导航" line-type="dashed"/>
        <n-timeline-item content="点击底部新导航按钮" line-type="dashed"/>
        <n-timeline-item content="移动到附近教室等地标" line-type="dashed" />
        <n-timeline-item content="输入信息" line-type="dashed"/>
        <n-timeline-item content="开始导航" line-type="dashed"/>
    </n-timeline>
    <n-timeline v-else>
        <n-timeline-item type="error" title="我的位置" :content="begin" :line-type="cur ? 'dashed' : 'default'"/>
        <n-timeline-item type="info" v-for="(item, i) in path" :content="item" :line-type="i < cur ? 'default' : 'dashed'"/>
        <n-timeline-item type="success" title="目的地" :content="end" line-type="dashed"/>
    </n-timeline>
</template>

<script lang="ts">
import { NTimeline, NTimelineItem } from 'naive-ui';
import { listen } from '@tauri-apps/api/event';

type Route = {
    start: string,
    dest: string,
    path: string[]
};


export default {
    components: {
        NTimeline, NTimelineItem
    },
    data() {
        return {
            begin: "",
            end: "",
            path: [] as string[],
            cur: 0
        };
    },
    methods: {
        async setPosistionListener() {
            await listen<number>('route-move', (event) => {
                this.cur += event.payload;
            });
        },
        async setRouteListener() {
            await listen<Route>('route-change', (event) => {
                this.begin = event.payload.start;
                this.end = event.payload.dest;
                this.path = event.payload.path;
                this.cur = 0;
            });
        }
    },
    mounted() {
        this.setRouteListener();
    },
};
</script>

<style scoped></style>