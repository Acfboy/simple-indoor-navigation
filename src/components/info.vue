<template>
    <n-card :bordered="false" title="Q&A">
        <n-h2> 这是什么？ </n-h2>
        <n-p>
            玩具项目，一个室内地图指引工具，可以帮忙找教室。
        </n-p>
        <n-h2> 为什么不做成网页？</n-h2>
        <n-p>
            你说得对，确实做成网页更方便。但是我当时不知道网页可以用 DeviceOrientationAbsolute 调用指南针，又想用指南针指引方向。
        </n-p>
        <n-h2> 会上架应用商店吗？</n-h2>
        <n-p> 不会，那比较麻烦。这只是个玩具项目，写着玩玩。 </n-p>
        <n-h2> 可以在哪些楼使用？ </n-h2>
        <n-p> 我录入了秦岭堂和陇山堂的数据，你也可以方便地录入其它楼。 </n-p>
    </n-card>

</template>

<script lang="ts">
import { NCard, NH2, NP } from 'naive-ui';

export default {
    components: {
        NCard, NH2, NP
    },
    data() {
        return {
        }
    },
    methods: {
    },
    mounted() {
    },
};
</script>
