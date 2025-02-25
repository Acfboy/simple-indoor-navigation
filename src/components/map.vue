<template>
    <canvas ref="canvas" :width="screenWidth
        " :height="screenHeight" v-if="image"></canvas>
    <n-empty description="导航未开始" style="position: relative; top:20%" v-else-if="imageUrl.length == 0">
    </n-empty>
    <n-spin size="large" style="position: relative; top:20%" v-else />
</template>

<script lang="ts">
import { NEmpty, NSpin } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { defineComponent, onMounted, ref } from "vue";

export default defineComponent({
    components: {
        NEmpty, NSpin
    },
    name: "Map",
    props: {
        screenWidth: {
            type: Number,
            default: 0
        },
        screenHeight: {
            type: Number,
            default: 0
        },
    },
    setup(props) {
        const imageUrl = ref<string>("");
        const angle = ref<number>(0);
        const newPoint = ref<{ x: number; y: number }>({ x: 0, y: 0 });
        const points = ref<{ x: number; y: number }[]>([{ x: 100, y: 100 }]);
        const image = ref<HTMLImageElement | null>(null);
        const canvas = ref<HTMLCanvasElement | null>(null);
        const offset = ref<{ x: number; y: number }>({ x: 0, y: 0 });
        const scale = ref<number>(1); 

        const loadImage = () => {
            image.value = null;
            const img = new Image();
            img.src = imageUrl.value;
            img.onload = () => {
                image.value = img;
                updateCanvas();
            };
        };

        const addPoint = () => {
            points.value.push({ ...newPoint.value });
            updateCanvas();
        };

        const setAngleListener = async () => {
            setInterval(async () => {
                type OrientationData = {
                    orientation: number;
                };
                const resp: OrientationData = await invoke("plugin:mobilesensors|get_orientation");
                angle.value = Math.floor(resp.orientation);
                updateCanvas();
            }, 200);
        }

        const pathLeftTop = (list: { x: number, y: number }[]) => {
            let leftTop = { x: 1e18, y: 1e18 };
            let rightDown = { x: 0, y: 0 };
            list.forEach((point) => {
                leftTop.x = Math.min(point.x);
                leftTop.y = Math.min(point.y);
                rightDown.x = Math.max(point.x);
                rightDown.y = Math.max(point.y);
            });
            leftTop.x -= (props.screenWidth - (rightDown.x - leftTop.x)) / 2;
            leftTop.y -= (props.screenHeight - (rightDown.y - leftTop.y)) / 2;
            return leftTop;
        }

        const setScaleListener = async() => {
            await listen<number>('update-scale', (event) => {
                scale.value = event.payload;
            });
        };

        const setPathListener = async () => {
            await listen<{ x: number, y: number }[]>('update-path', (event) => {
                const list = event.payload;
                offset.value = pathLeftTop(list);
                let newPoints: { x: number, y: number }[] = [];
                list.forEach((point) => {
                    newPoints.push({
                        x: point.x - offset.value.x,
                        y: point.y - offset.value.y
                    })
                });
                points.value = newPoints;
            });
        }

        const setImageListener = async () => {
            await listen<string>('update-image', (event) => {
                if (imageUrl.value == event.payload)
                    return;
                imageUrl.value = event.payload;
                loadImage();
            });
        }

        const updateCanvas = () => {
            if (!canvas.value || !image.value) return;
            const ctx = canvas.value.getContext("2d");
            if (!ctx) return;

            ctx.clearRect(0, 0, canvas.value.width, canvas.value.height);

            const centerX = canvas.value.width / 2;
            const centerY = canvas.value.height / 2;

            ctx.save();
            ctx.translate(centerX, centerY);
            ctx.rotate((angle.value * Math.PI) / 180);
            ctx.scale(scale.value, scale.value); // 使用 scale 变量进行缩放
            ctx.drawImage(image.value, -image.value.width / 2 - offset.value.x, -image.value.height / 2 - offset.value.y);
            ctx.restore();

            points.value.forEach((point) => {
                if (!image.value) return;
                const pointX = point.x - image.value.width / 2;
                const pointY = point.y - image.value.height / 2;

                const rotatedPointX =
                    centerX +
                    pointX * Math.cos((angle.value * Math.PI) / 180) -
                    pointY * Math.sin((angle.value * Math.PI) / 180);
                const rotatedPointY =
                    centerY +
                    pointX * Math.sin((angle.value * Math.PI) / 180) +
                    pointY * Math.cos((angle.value * Math.PI) / 180);

                ctx.beginPath();
                ctx.arc(rotatedPointX, rotatedPointY, 5, 0, Math.PI * 2);
                ctx.fillStyle = "red";
                ctx.fill();
            });
        };

        const setScreenSize = async () => {
            const size = Math.min(props.screenHeight, props.screenWidth);
            await invoke("set_screen_size", {
                x: size,
                y: size,
            })
        };

        onMounted(() => {
            setScreenSize();
            setAngleListener();
            setPathListener();
            setImageListener();
            setScaleListener();
        });

        return {
            imageUrl,
            angle,
            newPoint,
            points,
            image,
            canvas,
            addPoint,
            updateCanvas,
            scale, // 返回 scale 变量
        };
    },
});
</script>