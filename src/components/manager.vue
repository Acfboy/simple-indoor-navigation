<template>
    <n-card style="height: 100%;  overflow-y: auto;" title="地图管理" :bordered="false">
        <n-table :bordered="false">
            <tbody>
                <tr v-for="item in maps">
                    <td>
                        {{ item.label }}
                    </td>
                    <td>
                        <n-flex justify="end">
                            <n-button tertiary type="info" @click="exportMap(item.path, item.label)">
                                导出
                            </n-button>
                            <n-button tertiary type="error" @click="deleteMap(item.path)">
                                删除
                            </n-button>
                        </n-flex>
                    </td>
                </tr>
            </tbody>
        </n-table>
        <n-flex justify="end" style="padding-top: 20px;">
            <n-button tertiary type="success">
                导入地图
            </n-button>
        </n-flex>
    </n-card>
</template>

<script lang="ts">
import { NCard, NFlex, NTable, NButton } from 'naive-ui';
import { listen } from '@tauri-apps/api/event';
import { BaseDirectory, copyFile, readDir, remove } from '@tauri-apps/plugin-fs';
import { save } from '@tauri-apps/plugin-dialog';

type MapItem = {
    label: string,
    path: string,
}

export default {
    components: {
        NCard, NFlex, NTable, NButton
    },
    data() {
        return {
            maps: [] as MapItem[]
        };
    },
    methods: {
        async exportMap(path: string, name: string) {
            await copyFile(path, name + '.json', {
                fromPathBaseDir: BaseDirectory.AppData,
                toPathBaseDir: BaseDirectory.Download
            });
            alert('保存到下载目录下了')
        },
        async deleteMap(path: string) {
            await remove(path, {
                baseDir: BaseDirectory.AppData
            });
            this.refreshMap();
        },
        async refreshMap() {
            this.maps = [];
            const entries = await readDir("maps", {
                baseDir: BaseDirectory.AppData
            });
            for (const entry of entries) {
                this.maps.push({
                    label: entry.name.slice(0, -5),
                    path: 'maps/' + entry.name
                });
            }
        }
    },
    mounted() {
        this.refreshMap();
    },
};
</script>

<style scoped></style>