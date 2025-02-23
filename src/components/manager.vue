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
            <n-button tertiary type="success" @click="importMap()">
                导入地图
            </n-button>
        </n-flex>
    </n-card>
    <n-modal v-model:show="showSetName">
        <n-card :boarded="false" title="新地图名称">
            <n-input v-model:value="newName" />
            <n-flex justify="end">
                <n-button tertiary type="success" @click="setName">
                    确定
                </n-button>
            </n-flex>
        </n-card>
    </n-modal>
</template>

<script lang="ts">
import { NCard, NFlex, NTable, NButton, NModal, NInput } from 'naive-ui';
import { BaseDirectory, create, exists, mkdir, readDir, readTextFile, remove, writeTextFile } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';

type MapItem = {
    label: string,
    path: string,
}

export default {
    components: {
        NCard, NFlex, NTable, NButton, NModal, NInput
    },
    data() {
        return {
            maps: [] as MapItem[],
            showSetName: false,
            newName: "",
            newMapData: ""
        };
    },
    methods: {
        async importMap() {
            const dirExists = await exists('maps', {
                baseDir: BaseDirectory.AppData
            });
            if (!dirExists) {
                await mkdir('maps', {
                    baseDir: BaseDirectory.AppData,
                });
            }
            try {
                this.newName = "";
                this.newMapData = await invoke('import_map');
                this.showSetName = true;
            } catch {

            }
        },
        async setName() {
            try {
                const filename = 'maps/' + this.newName + '.json'
                await create(filename, {
                    baseDir: BaseDirectory.AppData
                });
                await writeTextFile(filename, this.newMapData, {
                    baseDir: BaseDirectory.AppData
                });
                this.refreshMap();
                this.showSetName = false;
            } catch (err) {
                alert(err);
            }
        },
        async exportMap(path: string, name: string) {
            const dirExists = await exists('maps', {
                baseDir: BaseDirectory.AppData
            });
            if (!dirExists) {
                await mkdir('maps', {
                    baseDir: BaseDirectory.AppData,
                });
            }
            let mapContent = await readTextFile(path, {
                baseDir: BaseDirectory.AppData
            });
            await invoke('export_map', {
                content: mapContent,
                name: name + '.json'
            })
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