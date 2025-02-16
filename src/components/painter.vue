<template>
  <n-layout>
    <n-layout-content :style="`height: ${screenHeight * 0.9}px`">
      <n-tabs type="line" animated style="padding: 0 20px 0 20px; height: 100%" @update:value="refreshMapList">
        <n-tab-pane name="add" tab="添加">
          <n-text type="info">
            {{ '前方方位角: ' + orien }}
          </n-text>
          <n-flex verticle>
            <n-card title="添加节点" :style="`height: ${screenHeight * 0.4}px; overflow-y: auto; `">
              添加路口每个方向的地标，然后添加节点。
              <n-table :bordered="false" :single-line="false" size="small">
                <thead>
                  <tr>
                    <th>方向</th>
                    <th>主名称</th>
                    <th>辅助名称</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in curInsec">
                    <td>{{ item.direction }}</td>
                    <td>{{ item.mark.name }}</td>
                    <td>{{ item.mark.detail }}</td>
                  </tr>
                </tbody>
              </n-table>
              <template #footer>
                <n-flex justify="end">
                  <n-button tertiary type="success" @click="showAddMarkToInsec">
                    添加地标
                  </n-button>
                  <n-button tertiary type="warning" @click="curInsecPop">
                    撤销添加
                  </n-button>
                  <n-button strong secondary type="primary" @click="addNode">
                    添加节点
                  </n-button>
                </n-flex>
              </template>
            </n-card>
            <n-card title="添加走廊" :style="`height: ${screenHeight * 0.4}px; overflow-y: auto;`">
              添加走廊中每个地标，确保两端地标在最前和最后，然后添加地标。
              <n-table :bordered="false" :single-line="false" size="small">
                <thead>
                  <tr>
                    <th>主名称</th>
                    <th>辅助名称</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in curCorri">
                    <td>{{ item.name }}</td>
                    <td>{{ item.detail }}</td>
                  </tr>
                </tbody>
              </n-table>
              <template #footer>
                <n-flex justify="end">
                  <n-button tertiary type="success" @click="showAddMarkToCorri">
                    添加地标
                  </n-button>
                  <n-button tertiary type="warning" @click="curCorriPop">
                    撤销添加
                  </n-button>
                  <n-button strong secondary type="primary" @click="addEdge">
                    添加走廊
                  </n-button>
                </n-flex>
              </template>
            </n-card>
          </n-flex>
        </n-tab-pane>
        <n-tab-pane name="del" tab="删除">
          <n-card title="删除节点或走廊" :style="`height: ${screenHeight * 0.8}px; overflow-y: auto;`">
            勾选要删除的内容，点击删除选中。
            <n-checkbox v-for="item in delList" v-model:checked="item.checked">
              {{ item.label }}
            </n-checkbox>
            <template #footer>
              <n-flex justify="end">
                <n-button secondary type="error" @click="delThat">
                  删除选中
                </n-button>
              </n-flex>
            </template>
          </n-card>
        </n-tab-pane>
      </n-tabs>
    </n-layout-content>
    <n-layout-footer bordered>
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
    </n-layout-footer>
  </n-layout>

  <n-modal v-model:show="addMarkWithDire">
    <n-card style="width: 90%" title="添加地标" :bordered="false" size="huge" role="dialog" aria-modal="true">
      <n-flex vertical>
        方位角
        <n-input type="text" size="small" :placeholder="String(orien)" :disabled="true" />
        主名称
        <n-input v-model:value="curMark.name" placeholder="主名称" type="text" />
        辅助名称（在主名称相同时用来区分）
        <n-input v-model:value="curMark.detail" placeholder="辅助名称" type="text" />
        层数（若不是电梯/楼梯，保持 0）
        <n-input-number v-model:value="curMark.elevatorFloor" />
        <n-flex justify="end">
          <n-button type="success" @click="addMarkToInsec">
            添加
          </n-button>
        </n-flex>
      </n-flex>
    </n-card>
  </n-modal>

  <n-modal v-model:show="addMark">
    <n-card style="width: 90%" title="添加地标" :bordered="false" size="huge" role="dialog" aria-modal="true">
      <n-flex vertical>
        主名称
        <n-input v-model:value="curMark.name" placeholder="主名称" type="text" />
        辅助名称（在主名称相同时用来区分）
        <n-input v-model:value="curMark.detail" placeholder="辅助名称" type="text" />
        <n-flex justify="end">
          <n-button type="success" @click="addMarkToCorri">
            添加
          </n-button>
        </n-flex>
      </n-flex>
    </n-card>
  </n-modal>

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
import {
  NCard, NLayout, NLayoutContent, NLayoutFooter, NFlex, NIcon, NText, NLayoutHeader, NTabs, NTabPane, NForm,
  NFormItem, NInput, NCheckbox, NButton, NTable, NModal, NInputNumber, NTransfer, useMessage,
} from 'naive-ui';
import { exists, BaseDirectory, create, writeTextFile, mkdir } from '@tauri-apps/plugin-fs';
import { ArrowBack, SaveOutline } from '@vicons/ionicons5';
import { invoke } from '@tauri-apps/api/core';

type Corridor = Mark[];

type Mark = {
  name: string,
  detail: string,
  elevatorFloor: number
}

type Branch = {
  direction: number,
  mark: Mark
}

type Intersection = Branch[];

type OriginalMap = {
  nodes: Intersection[],
  edges: Corridor[]
}

type CheckboxItem = {
  label: string,
  type: string,
  id: number,
  checked: false
}

type CheckboxList = CheckboxItem[];


function markName(x: Mark) {
  let s: string = '';
  s = x.name;
  if (x.detail.length) s += '(' + x.detail + ')';
  if (x.elevatorFloor) s += x.elevatorFloor + '层';
  return s;
}

export default {
  components: {
    NCard, NLayout, NLayoutContent, NLayoutFooter, NFlex, NIcon, SaveOutline,
    ArrowBack, NText, NLayoutHeader, NTabs, NTabPane, NForm, NFormItem,
    NInput, NCheckbox, NButton, NTable, NModal, NInputNumber, NTransfer, useMessage
  },
  props: ['screenHeight'],
  data() {
    return {
      orien: 0,
      map: {
        nodes: [],
        edges: []
      } as OriginalMap,
      curMark: {
        name: "",
        detail: "",
        elevatorFloor: 0
      } as Mark,
      curInsec: [] as Intersection,
      curFloor: "",
      addMarkWithDire: false,
      curCorri: [] as Corridor,
      addMark: false,
      delOptions: [],
      delList: [] as CheckboxList,
      showSaveMap: false,
      mapName: ""
    }
  },
  methods: {
    async saving() {
      const dirExists = await exists('maps', {
        baseDir: BaseDirectory.AppData
      });
      const filename = 'maps/' + this.mapName + '.json';
      const fileExists = await exists(filename, {
        baseDir: BaseDirectory.AppData,
      });
      if (!dirExists) {
        await mkdir('maps', {
          baseDir: BaseDirectory.AppData,
        });
      }
      if (!fileExists) {
        await create(filename, { baseDir: BaseDirectory.AppData });
      }
      const contents = JSON.stringify(this.map);
      await writeTextFile(filename, contents, {
        baseDir: BaseDirectory.Data,
      });
    },
    async saveMap() {
      if (this.mapName.length) {
        try {
          await this.saving();
          alert("保存成功")
          this.handleBack();
        }
        catch (err) {
          alert(err);
        }
      }
      else {
        alert('地图名称不能为空');
      }
    },
    toSaveMap() {
      this.showSaveMap = true;
    },
    delThat() {
      let nodesDelId = [];
      let edgesDelId = [];
      for (let cur of this.delList) {
        if (cur.checked) {
          if (cur.type == "insec")
            nodesDelId.push(cur.id);
          else edgesDelId.push(cur.id);
        }
      }
      nodesDelId.sort((a, b) => b - a);
      edgesDelId.sort((a, b) => b - a);
      for (let i of nodesDelId) {
        this.map.nodes.splice(i, 1);
      }
      for (let i of edgesDelId) {
        this.map.edges.splice(i, 1);
      }
      this.refreshMapList();
    },
    refreshMapList() {
      this.delList = []
      this.delList = this.delList.concat(this.map.nodes.map((cur: Intersection, index: number) => {
        return {
          label: markName(cur[0].mark) + ' 附近路口',
          type: "insec",
          id: index,
          checked: false
        }
      }));
      this.delList = this.delList.concat(this.map.edges.map((cur: Corridor, index: number) => {
        return {
          label: markName(cur[0]) + ' 至 ' + markName(cur[cur.length - 1]) + ' 走廊',
          type: "corri",
          id: index,
          checked: false
        }
      }));
    },
    addNode() {
      if (this.curInsec.length == 0) {
        alert("不能添加空节点");
      } else {
        let arr = new Array();
        let set = new Set();
        for (const c of this.curInsec) {
          arr.push(c.mark.name + c.mark.detail);
          set.add(c.mark.name + c.mark.detail);
        }
        if (arr.length != set.size) {
          alert('不同地标请设置不同的名称。');
        } else {
          this.map.nodes.push(this.curInsec);
        }
      }
      this.curInsec = []
    },
    addEdge() {
      if (this.curCorri.length == 0) {
        alert("不能添加空走廊");
      }
      else this.map.edges.push(this.curCorri);
      this.curCorri = []
    },
    curInsecPop() {
      this.curInsec.pop();
    },
    curCorriPop() {
      this.curCorri.pop();
    },
    handleBack() {
      this.$emit('switch', 'main')
    },
    addMarkToInsec() {
      const curBranch: Branch = {
        direction: this.orien,
        mark: this.curMark
      };
      if (this.curMark.elevatorFloor > 163 || this.curMark.elevatorFloor < -163) {
        alert('世界最高楼哈利法塔的层数是 163，我觉得你这层数不对。');
      }
      else {
        this.curInsec.push(curBranch);
        this.addMarkWithDire = false;
      }
    },
    showAddMarkToInsec() {
      this.curMark = {
        name: "",
        detail: "",
        elevatorFloor: 0
      };
      this.addMarkWithDire = true;
    },
    addMarkToCorri() {
      this.curCorri.push(this.curMark);
      this.addMark = false;
    },
    showAddMarkToCorri() {
      this.curMark = {
        name: "",
        detail: "",
        elevatorFloor: 0
      };
      this.addMark = true;
    }
  },
  mounted() {
    setInterval(async () => {
      type OrientationData = {
        orientation: number;
      };
      const resp: OrientationData = await invoke("plugin:mobilesensors|get_orientation");
      this.orien = Math.floor(resp.orientation);
    }, 200);
  },
};
</script>

<style scoped>
.n-layout-footer {
  padding: 24px;
  background-color: white;
}
</style>