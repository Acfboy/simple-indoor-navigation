<template>
    <div v-show="image && !climbing">
        <canvas ref="canvas" :width="screenWidth
            " :height="screenHeight"></canvas>
    </div>
    <n-empty description="导航未开始" style="position: relative; top:40%" v-if="imageUrl.length == 0 && !climbing">
    </n-empty>
    <n-flex justify="center" style="position: relative; top:40%;">
        <n-icon size="5em" v-if="climbing">
            <CaretUpCircleOutline />
            <CaretDownCircleOutline />
        </n-icon>
    </n-flex>
    <n-spin size="large" style="position: relative; top:20%" v-if="imageUrl.length != 0 && !image" />
</template>

<script lang="ts">
import { NEmpty, NSpin, NIcon, NFlex } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { defineComponent, onMounted, ref } from "vue";
import { CaretUpCircleOutline, CaretDownCircleOutline } from "@vicons/ionicons5"

export default defineComponent({
    components: {
        NEmpty, NSpin, NIcon, CaretUpCircleOutline, CaretDownCircleOutline, NFlex
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
        const topLeft = ref< { x: number; y: number}>({x:0, y:0});
        const climbing = ref<boolean>(false);
        let spanx = 0;
        let spany = 0;

        const calcOffset = () => {
            offset.value.x = -topLeft.value.x * scale.value + (props.screenWidth - spanx * scale.value) / 2;
            offset.value.y = -topLeft.value.y * scale.value + (props.screenHeight - spany * scale.value) / 2;
        }

        const loadImage = () => {
            image.value = null;
            const img = new Image();
            img.src = imageUrl.value;
            img.onload = () => {
                image.value = img;
                updateCanvas();
                calcOffset();
            };
        };

        const setAngleListener = async () => {
            setInterval(async () => {
                type OrientationData = {
                    orientation: number;
                };
                const resp: OrientationData = await invoke("plugin:mobilesensors|get_orientation");
                angle.value = -Math.floor(resp.orientation);
                updateCanvas();
            }, 100);
        }

        const pathLeftTop = (list: { x: number, y: number }[]) => {
            let leftTop = { x: 1e18, y: 1e18 };
            let rightDown = { x: 0, y: 0 };
            list.forEach((point) => {
                leftTop.x = Math.min(point.x, leftTop.x);
                leftTop.y = Math.min(point.y, leftTop.y);
                rightDown.x = Math.max(point.x, rightDown.x);
                rightDown.y = Math.max(point.y, rightDown.y);
            });
            spanx = rightDown.x - leftTop.x;
            spany = rightDown.y - leftTop.y;
            // leftTop.x = -leftTop.x + (props.screenWidth - (rightDown.x - leftTop.x)) / 2;
            // leftTop.y = -leftTop.y + (props.screenHeight - (rightDown.y - leftTop.y)) / 2;
            // alert(JSON.stringify(leftTop))
            return leftTop;
        }

        const setScaleListener = async () => {
            await listen<number>('update-scale', (event) => {
                // alert('new scale' + event.payload)
                scale.value = event.payload;
            });
        };

        type Node = {
            name: string,
            pos: { x: number, y: number },
            floor: number,
            elevator: string,
            index: number,
        }

        const setPathListener = async () => {
            await listen<Node[]>('update-route', (event) => {
                const list = event.payload.map((node) => node.pos);
                topLeft.value = pathLeftTop(list);
                let newPoints: { x: number, y: number }[] = [];
                list.forEach((point) => {
                    newPoints.push({
                        x: point.x,
                        y: point.y
                    })
                });
                points.value = newPoints;
                calcOffset();
                updateCanvas();
            });
        }

        const setImageListener = async () => {
            await listen<string>('update-image', (event) => {
                if (imageUrl.value == event.payload)
                    return;
                imageUrl.value = event.payload;

                if (imageUrl.value.length == 0)
                    climbing.value = true;
                else {
                    climbing.value = false;
                    loadImage();
                }
            });
        }

        const updateCanvas = () => {
            if (!canvas.value || !image.value) return;
            const ctx = canvas.value.getContext("2d");
            if (!ctx) return;

            ctx.clearRect(0, 0, canvas.value.width, canvas.value.height);

            const centerX = canvas.value.width / 2;
            const centerY = canvas.value.height / 2;
            const angleRad = (angle.value * Math.PI) / 180;

            // 绘制图片
            ctx.save();
            ctx.translate(centerX, centerY); // 移动坐标系到画布中心
            ctx.rotate(angleRad); // 绕中心旋转
            // 绘制图片（offset是相对画布左上角的偏移，需转换为中心坐标系下的坐标）
            ctx.drawImage(
                image.value,
                offset.value.x - centerX, // 计算图片左上角在旋转坐标系中的X位置
                offset.value.y - centerY,  // 计算图片左上角在旋转坐标系中的Y位置
                image.value.width * scale.value,
                image.value.height * scale.value
            );
            ctx.restore();

            // 绘制红点
            const cos = Math.cos(angleRad);
            const sin = Math.sin(angleRad);

            // 先计算所有旋转后的坐标并存储
            const rotatedPoints = points.value.map(point => {
                const rawX = offset.value.x + point.x * scale.value;
                const rawY = offset.value.y + point.y * scale.value;

                // 转换为绕中心旋转的坐标
                let x = rawX - centerX;
                let y = rawY - centerY;

                return {
                    x: x * cos - y * sin + centerX,
                    y: x * sin + y * cos + centerY
                };
            });

            // 绘制连接线（带箭头）
            if (rotatedPoints.length >= 2) {
                ctx.strokeStyle = "blue";
                ctx.fillStyle = "blue";
                ctx.lineWidth = 2;

                for (let i = 0; i < rotatedPoints.length - 1; i++) {
                    const start = rotatedPoints[i];
                    const end = rotatedPoints[i + 1];

                    // 绘制线段
                    ctx.beginPath();
                    ctx.moveTo(start.x, start.y);
                    ctx.lineTo(end.x, end.y);
                    ctx.stroke();

                    // 绘制箭头
                    const angle = Math.atan2(end.y - start.y, end.x - start.x);
                    ctx.save();
                    ctx.translate(end.x, end.y);
                    ctx.rotate(angle);
                    ctx.beginPath();
                    ctx.moveTo(0, 0);
                    ctx.lineTo(-10, 5);
                    ctx.lineTo(-10, -5);
                    ctx.closePath();
                    ctx.fill();
                    ctx.restore();
                }
            }

            // 绘制点（先画线后画点保证点在上层）
            rotatedPoints.forEach((point, index) => {
                ctx.beginPath();
                ctx.arc(point.x, point.y, 5, 0, Math.PI * 2);
                ctx.fillStyle = index === 0 ? "red" :
                    index === rotatedPoints.length - 1 ? "green" : "blue";
                ctx.fill();
            });
        };


        onMounted(() => {
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
            updateCanvas,
            scale, // 返回 scale 变量
            climbing,
        };
    },
});
</script>