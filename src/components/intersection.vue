<template>
    <n-card bordered title="下一个路口参照物" size="small" style="height: 100%;">
        <n-list bordered v-if="landmarkList.length != 0">
            <n-list-item v-for="item in landmarkList">
                {{ item }}
            </n-list-item>
        </n-list>
        <n-empty description="导航未开始" style="position: relative; top:20%" v-else>
        </n-empty>
    </n-card>
</template>

<script lang="ts">
import { NCard, NEmpty, NList, NListItem } from 'naive-ui';
import { listen } from '@tauri-apps/api/event';

type IntersectionLandmarks = {
    landmarks: string[],
};


export default {
    components: {
        NCard,
        NEmpty,
        NList,
        NListItem
    },
    data() {
        return {
            landmarkList: [] as string[],
        };
    },
    methods: {
        async setLandmarkListener() {
            await listen<IntersectionLandmarks>('landmark-change', (event) => {
                this.landmarkList = event.payload.landmarks;
            });
        }
    },
    mounted() {
        this.setLandmarkListener();
    },
};
</script>

<style scoped>
</style>