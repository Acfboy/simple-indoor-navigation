<template>
    <n-layout>
        <n-layout-header :style="`height: ${screenHeight * 0.5}px;`" bordered>
            <div v-for="(item, index) in images">
                <Editmap v-show="index + 1 == floor" :image-url="item" :mapHeight="screenHeight * 0.5"
                    :mapWidth="screenWidth" :type="editType" :cur-floor="index + 1" @scale="updateScale(index, $event)"
                    @select-node="handleSelectNode" :init-data="initDataList.length ? initDataList[index] : {}" />
            </div>
        </n-layout-header>
        <n-layout-content :style="`height: ${screenHeight * 0.5}px`">
            <n-flex vertical>
                <n-tabs v-model:value="editType" @update:value="updateNodes">
                    <n-tab name="add" tab="添加"></n-tab>
                    <n-tab name="del" tab="删除"></n-tab>
                    <n-tab name="mark" tab="标注"></n-tab>
                </n-tabs>

                <div v-show="images.length != 0">
                    <n-pagination v-model:page="floor" :page-count="images.length" @update:page="markNode" />
                </div>

                <n-button secondary type="info" v-if="images.length == 0" @click="loadFromFile">
                    从已有地图开始
                </n-button>

                <n-button secondary @click="addFloor">
                    添加一层
                </n-button>

                <n-switch v-model:value="autofill">
                    <template #checked>
                        自动填充开
                    </template>
                    <template #unchecked>
                        自动填充关
                    </template>
                </n-switch>

                <n-card v-if="editType == 'mark'" title="标注选中点">
                    <div v-for="item in nodeList">
                        <!-- {{ item }} -->
                        <div v-if="item.index == selectedIndex">
                            节点名称 <n-input v-model:value="item.name" type="text" placeholder="节点名称"
                                @on-blur="markNode" />
                            节点名称 <n-input v-model:value="item.elevator" type="text" placeholder="电梯/楼梯名 非电梯/楼梯则留空"
                                @on-blur="markNode" />
                        </div>
                    </div>
                </n-card>

                <n-flex justify="space-between">
                    <n-button @click="handleBack">
                        <template #icon>
                            <n-icon>
                                <ArrowBack />
                            </n-icon>
                        </template>
                        返回
                    </n-button>
                    <n-button @click="toSaveMap" size="small">
                        <template #icon>
                            <n-icon>
                                <SaveOutline />
                            </n-icon>
                        </template>
                        保存地图
                    </n-button>
                </n-flex>
            </n-flex>
        </n-layout-content>
    </n-layout>

    <n-modal v-model:show="showSaveMap">
        <n-card style="width: 90%" title="保存地图" :bordered="false" size="huge" role="dialog" aria-modal="true">
            <n-flex vertical>
                地图名称
                <n-input v-model:value="mapName" placeholder="地图名称" type="text" />
                <n-flex justify="end">
                    <n-button type="success" @click="saveMap">
                        保存
                    </n-button>
                </n-flex>
            </n-flex>
        </n-card>
    </n-modal>

    <n-modal v-model:show="selectMap">
        <n-card style="width: 90%" title="选择初始地图" :bordered="false" size="huge" role="dialog" aria-modal="true">
            <n-flex verticle>
                <n-select v-model:value="selectedOriginName" filterable placeholder="地图" :options="maps" />
                <n-flex justify="end">
                    <n-button type="success" @click="loadMap">
                        确认
                    </n-button>
                </n-flex>
            </n-flex>
        </n-card>
    </n-modal>
</template>

<script lang="ts">
import { NLayout, NLayoutContent, NLayoutHeader, NTabs, NTab, NPagination, NButton, NInput, NFlex, NCard, NModal, NIcon, NSelect, NSwitch } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import { defineComponent, onMounted, ref } from "vue";
import Editmap from "./painter/editmap.vue";
import { BaseDirectory, create, exists, mkdir, readDir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { ArrowBack, SaveOutline } from "@vicons/ionicons5";

export default defineComponent({
    name: "Map",
    components: {
        NLayout, Editmap, NLayoutHeader, NModal, NLayoutContent, NSelect, NSwitch,
        NTabs, NTab, NPagination, NButton, NInput, NFlex, NCard, NIcon, ArrowBack, SaveOutline
    },
    emits: ['switch'],
    setup(_, context) {
        const screenHeight = ref(window.innerHeight);
        const screenWidth = ref(window.innerWidth);
        const editType = ref("add");
        const floorNumber = ref(0);
        const selectedOriginName = ref("");
        const selectMap = ref(false);
        const maps = ref<{ label: string, value: string }[]>([]);
        const floor = ref(1);
        const selectedIndex = ref(-1);
        const autofill = ref(false);

        type Position = {
            x: number,
            y: number
        };

        type Node = {
            name: string,
            pos: Position,
            floor: number,
            elevator: string,
            index: number,
        }

        interface Block {
            x: number
            y: number
            size: number
            index: number
        }

        interface Line {
            start: Block
            end: Block
        }

        interface InitData {
            blocks: Block[],
            lines: Line[],
        }


        const nodeList = ref<Node[]>([]);
        const images = ref<string[]>([]);
        const mapName = ref<string>("");
        const initDataList = ref<InitData[]>([]);

        const updateNodes = async () => {
            await markNode();
            nodeList.value = await invoke('cur_node_list');
        }

        const markNode = async () => {
            const cur = nodeList.value.find(e => e.index == selectedIndex.value);
            if (cur) {
                // alert(cur.name);
                await invoke('mark_node', {
                    index: cur.index,
                    name: cur.name,
                    elevator: cur.elevator
                });
            }
        }

        const loadMap = async () => {
            try {
                const dirExists = await exists('maps', {
                    baseDir: BaseDirectory.AppData
                });
                if (!dirExists) {
                    await mkdir('maps', {
                        baseDir: BaseDirectory.AppData,
                    });
                }
                const content = await readTextFile("maps/" + selectedOriginName.value + '.json', {
                    baseDir: BaseDirectory.AppData
                });
                const mapObj = JSON.parse(content);
                await invoke('load_from_file', {
                    mapData: mapObj.map
                });
                updateNodes();
                let dataList: InitData[] = [];
                // initDataList.value = [];
                for (const _ in mapObj.imgs) {
                    dataList.push({
                        lines: [],
                        blocks: [],
                    })
                }

                type OriNodeType = {
                    name: string,
                    pos: { x: number, y: number },
                    floor: number,
                    elevator: string,
                    index: number
                };

                let convert = (c: OriNodeType) => {
                    return {
                        x: c.pos.x,
                        y: c.pos.y,
                        index: c.index,
                        size: 20,
                    };
                }

                for (const c of mapObj.map.nodes)
                    dataList[c.floor - 1].blocks.push(convert(c));
                for (const i in mapObj.map.edges) {
                    for (const v of mapObj.map.edges[i])
                        dataList[mapObj.map.nodes[i].floor - 1].lines.push({
                            start: convert(mapObj.map.nodes[i]),
                            end: convert(mapObj.map.nodes[v])
                        });
                    // if (dataList[0].lines[dataList[0].lines.length - 1].end == undefined) {
                    //     alert(mapObj.map.edges[i])
                    // }
                }
                initDataList.value = dataList;
                images.value = mapObj.imgs;
                selectMap.value = false;
            } catch (e) {
                alert(e)
            }
        };

        const loadFromFile = async () => {
            const entries = await readDir("maps", {
                baseDir: BaseDirectory.AppData
            });
            let mapList = [];
            for (const entry of entries) {
                mapList.push(entry.name.slice(0, -5))
            }
            maps.value = mapList.map((s) => {
                return {
                    label: s,
                    value: s
                }
            });
            selectMap.value = true;
        };


        const handleBack = () => {
            context.emit('switch', 'main')
        };

        const scales = ref<number[]>([]);

        const updateScale = (index: number, payload: number) => {
            scales.value[index] = payload;
        };

        const execSave = async (mapName: string, data: string) => {
            const dirExists = await exists('maps', {
                baseDir: BaseDirectory.AppData
            });
            if (!dirExists) {
                await mkdir('maps', {
                    baseDir: BaseDirectory.AppData,
                });
            }
            const filename = 'maps/' + mapName + '.json';
            const fileExists = await exists(filename, {
                baseDir: BaseDirectory.AppData,
            });
            if (!fileExists) {
                await create(filename, { baseDir: BaseDirectory.AppData });
            }
            await writeTextFile(filename, data, {
                baseDir: BaseDirectory.AppData,
            });
        }

        const saveMap = async () => {
            if (mapName.value.length) {
                try {
                    let data: string = await invoke("get_store_data", {
                        imgs: images.value,
                        scales: scales.value
                    });
                    execSave(mapName.value, data);
                    alert("保存成功")
                    handleBack();
                }
                catch (err) {
                }
            }
            else {
                alert('地图名称不能为空');
            }
        };

        const showSaveMap = ref(false);

        const toSaveMap = () => {
            markNode();
            showSaveMap.value = true;
        };

        const addFloor = async () => {
            try {
                const payload: { 0: number[], 1: string } = await invoke('select_image');
                // alert(typeof payload[0][0])
                const binaryData = new Uint8Array(payload[0]);
                const binary = new Blob([binaryData], { type: payload[1] });
                let reader = new FileReader();
                reader.readAsDataURL(binary);
                reader.onload = () => {
                    if (typeof reader.result === 'string') {
                        // alert(reader.result);
                        images.value.push(reader.result);
                        scales.value.push(1);
                    }
                    else throw new Error("图片没读出");
                }
            } catch (e) {
            }
        }

        const handleSelectNode = async (index: number) => {
            await markNode();
            const fill = (s: string) => {
                let p = 0;
                while (p < s.length && isNaN(Number(s.slice(p))))
                    p += 1;
                if (p == s.length) return s;
                let nu = Number(s.slice(p));
                return s.slice(0, p) + (nu + 1);
            };
            while (nodeList.value.length <= index) {
                nodeList.value.push({
                    name: "",
                    pos: { x: 0, y: 0 },
                    floor: 0,
                    elevator: "",
                    index: nodeList.value.length,
                })
            }
            if (selectedIndex.value != null && autofill.value && nodeList.value[index].name.length == 0) {
                nodeList.value[index].name = fill(nodeList.value[selectedIndex.value].name);
                // alert()
            }
            selectedIndex.value = index;
            await markNode();
        };

        onMounted(async () => {
            await invoke("clear_data");
        });

        return {
            screenHeight,
            screenWidth,
            editType,
            updateNodes,
            floor,
            floorNumber,
            addFloor,
            images,
            nodeList,
            selectedIndex,
            markNode,
            showSaveMap,
            mapName,
            saveMap,
            handleBack,
            toSaveMap,
            updateScale,
            handleSelectNode,
            selectedOriginName,
            selectMap,
            maps,
            loadFromFile,
            loadMap,
            initDataList,
            autofill
        };
    },
});
</script>