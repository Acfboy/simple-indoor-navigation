<template>
  <n-layout>
    <n-layout-header :style="`height: ${screenHeight * 0.5}px;`" bordered>
      <div v-for="(item, index) in images">
        <Editmap v-show="index + 1 == floor" :image-url="item" :mapHeight="screenHeight * 0.5" :mapWidth="screenWidth"
          :type="editType" :cur-floor="index + 1" @scale="updateScale(index, $event)" @select-node="handleSelectNode"/>
      </div>
    </n-layout-header>
    <n-layout-content :style="`height: ${screenHeight * 0.4}px`">
      <n-flex vertical >
        <n-tabs v-model:value="editType" @update:value="updateNodes">
          <n-tab name="add" tab="添加"></n-tab>
          <n-tab name="del" tab="删除"></n-tab>
          <n-tab name="mark" tab="标注"></n-tab>
        </n-tabs>

        <div v-show="images.length != 0">
          <n-pagination v-model:page="floor" :page-count="images.length" @update:page="markNode"/>
        </div>

        <n-button secondary @click="addFloor">
          添加一层
        </n-button>
        <n-card v-if="editType == 'mark'" title="标注选中点">
          <div v-for="item in nodeList">
            <!-- {{ item }} -->
            <div v-if="item.index == selectedIndex">
              节点名称 <n-input v-model:value="item.name" type="text" placeholder="节点名称" @on-blur="markNode" />
              节点名称 <n-input v-model:value="item.elevator" type="text" placeholder="电梯/楼梯名 非电梯/楼梯则留空"
                @on-blur="markNode" />
            </div>
          </div>
        </n-card>
        <n-flex justify="space-between">
          <n-button @click="handleBack" text>
            <n-icon>
              <ArrowBack />
            </n-icon>
            返回
          </n-button>
          <n-button @click="toSaveMap" text size="small">
            <n-icon>
              <SaveOutline />
            </n-icon>
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
</template>

<script lang="ts">
import { NLayout, NLayoutContent, NLayoutHeader, NTabs, NTab, NPagination, NButton, NInput, NFlex, NCard, NModal, NIcon } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import { defineComponent, onMounted, ref } from "vue";
import Editmap from "./painter/editmap.vue";
import { BaseDirectory, create, exists, mkdir, writeTextFile } from "@tauri-apps/plugin-fs";
import { ArrowBack, SaveOutline } from "@vicons/ionicons5";



export default defineComponent({
  name: "Map",
  components: {
    NLayout, Editmap, NLayoutHeader, NModal, NLayoutContent,
    NTabs, NTab, NPagination, NButton, NInput, NFlex, NCard, NIcon, ArrowBack, SaveOutline
  },
  emits: ['switch'],
  setup(_, context) {
    const screenHeight = ref(window.innerHeight);
    const screenWidth = ref(window.innerWidth);
    const editType = ref("add");
    const floorNumber = ref(0);
    const floor = ref(1);
    const selectedIndex = ref(-1);

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

    const nodeList = ref<Node[]>([]);
    const images = ref<string[]>([]);
    const mapName = ref<string>("");

    const updateNodes = async () => {
      nodeList.value = await invoke('cur_node_list');
    }

    const markNode = async () => {
      const cur = nodeList.value.find(e => e.index == selectedIndex.value);
      if (cur) {
        await invoke('mark_node', {
          index: cur.index,
          name: cur.name,
          elevator: cur.elevator
        });
      }
    }


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

    const handleSelectNode = (index: number) => {
        markNode();
        selectedIndex.value = index;
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
      handleSelectNode
    };
  },
});
</script>