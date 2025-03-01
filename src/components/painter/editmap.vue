<template>
    <canvas ref="canvas" @touchstart="handleTouchStart" @touchmove="handleTouchMove" @touchend="handleTouchEnd"
        @touchcancel="handleTouchEnd" :height="mapHeight" :width="mapWidth"></canvas>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { defineComponent, ref, onMounted } from 'vue'

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

// interface initData {
//     blocks: Block[],
//     lines: Line[],
// }

export default defineComponent({
    props: {
        mapHeight: {
            type: Number,
            default: 1
        },
        mapWidth: {
            type: Number,
            defalut: 1
        },
        imageUrl: {
            type: String,
            default: ""
        },
        type: {
            type: String,
            default: ""
        },
        curFloor: {
            type: Number,
            default: 0
        },
        initData: {
            type: Object,
            default: {}, 
        }
    },
    setup(props, context) {
        const canvas = ref<HTMLCanvasElement | null>(null)
        const ctx = ref<CanvasRenderingContext2D | null>(null)
        const image = new Image()
        const blocks = ref<Block[]>([])
        const lines = ref<Line[]>([])
        const selectedBlock = ref<Block | null>(null)
        const justClicked = ref<Block | null>(null)
        const lastType = ref<string>("null");

        // 视图参数
        const scale = ref(1)
        const offset = ref({ x: 0, y: 0 })
        const lastTouch = ref<{
            time: number
            x: number
            y: number
        } | null>(null)

        // 触摸状态
        const touchState = ref({
            isDragging: false,
            startX: 0,
            startY: 0,
            pinchDistance: 0,
            initialOffset: { x: 0, y: 0 },
            initialScale: 1
        })

        // 坐标转换
        const   getCanvasPosition = (clientX: number, clientY: number) => {
            if (!canvas.value) return { x: 0, y: 0 }
            const rect = canvas.value.getBoundingClientRect()
            return {
                x: (clientX - rect.left - offset.value.x) / scale.value,
                y: (clientY - rect.top - offset.value.y) / scale.value
            }
        }

        // 绘制方法
        const draw = () => {
            if (!ctx.value || !canvas.value) return

            ctx.value.clearRect(0, 0, canvas.value.width, canvas.value.height)
            ctx.value.save()
            ctx.value.setTransform(scale.value, 0, 0, scale.value, offset.value.x, offset.value.y)

            ctx.value.drawImage(image, 0, 0)
            blocks.value.forEach(block => {
                if (props.type == 'mark' && justClicked.value && block.index == justClicked.value.index) {
                    ctx.value!.fillStyle = '#0000ff'
                } else {
                    ctx.value!.fillStyle = '#ff0000'
                }
                
                ctx.value!.fillRect(
                    block.x - block.size / 2,
                    block.y - block.size / 2,
                    block.size,
                    block.size
                )
            })

            ctx.value!.strokeStyle = '#00ff00'
            ctx.value!.lineWidth = 2 / scale.value
            lines.value.forEach(line => {
                ctx.value!.beginPath()
                ctx.value!.moveTo(line.start.x, line.start.y)
                ctx.value!.lineTo(line.end.x, line.end.y)
                ctx.value!.stroke()
            })

            ctx.value!.restore()
        }



        // 处理触摸开始（新增连线逻辑）
        const handleTouchStart = async (e: TouchEvent) => {
            if (lastType.value != props.type) {
                selectedBlock.value = null;
                lastType.value = props.type;
            }
            if (!canvas.value) return
            e.preventDefault()

            const now = Date.now()
            const touch = e.touches[0]
            const pos = getCanvasPosition(touch.clientX, touch.clientY)

            // 检测是否点击到方块
            const clickedBlock = blocks.value.find(b =>
                Math.abs(pos.x - b.x) < b.size &&
                Math.abs(pos.y - b.y) < b.size
            )

            // 双击添加方块
            if (e.touches.length === 1) {
                if (lastTouch.value && now - lastTouch.value.time < 300 &&
                    Math.abs(pos.x - lastTouch.value.x) < 30 &&
                    Math.abs(pos.y - lastTouch.value.y) < 30) {

                    if (props.type == 'add') {
                        const newIndex: number = await invoke('add_node', {
                            pos: pos,
                            floor: props.curFloor,
                        });
                        blocks.value.push({
                            x: pos.x,
                            y: pos.y,
                            size: 20,
                            index: newIndex
                        })
                    } else if (props.type == 'del') {
                        if (clickedBlock) {
                            await invoke('remove_node', {
                                index: clickedBlock.index
                            })
                            blocks.value = blocks.value.filter(x =>
                                x.index != clickedBlock.index
                            )
                            lines.value = lines.value.filter(x =>
                                !(x.start.index == clickedBlock.index || x.end.index == clickedBlock.index)
                            );
                        }
                    }

                    lastTouch.value = null
                    draw()
                    return
                }
                lastTouch.value = { time: now, x: pos.x, y: pos.y }
            }

            if (clickedBlock) {

                if (props.type == 'mark') {
                    context.emit('select-node', clickedBlock.index);
                    justClicked.value = clickedBlock;
                }

                if (selectedBlock.value) {

                    if (props.type == 'add') {
                        await invoke('add_edge', {
                            from: selectedBlock.value.index,
                            to: clickedBlock.index
                        });
                        lines.value.push({
                            start: selectedBlock.value,
                            end: clickedBlock
                        })
                    } else if (props.type == 'del') {
                        const u = selectedBlock.value.index;
                        const v = clickedBlock.index;
                        await invoke('remove_edge', {
                            from: u,
                            to: v
                        });
                        lines.value = lines.value.filter(x =>
                            !(x.start.index == u && x.end.index == v ||
                                x.start.index == v && x.end.index == u)
                        );
                    }

                    selectedBlock.value = null
                } else {
                    selectedBlock.value = clickedBlock
                }
                draw()
                return
            }

            // 处理手势
            if (e.touches.length === 1) {
                touchState.value = {
                    ...touchState.value,
                    isDragging: true,
                    startX: touch.clientX,
                    startY: touch.clientY,
                    initialOffset: { ...offset.value }
                }
            }

            if (e.touches.length === 2) {
                const touch1 = e.touches[0]
                const touch2 = e.touches[1]
                const distance = Math.hypot(
                    touch2.clientX - touch1.clientX,
                    touch2.clientY - touch1.clientY
                )
                touchState.value = {
                    ...touchState.value,
                    pinchDistance: distance,
                    initialScale: scale.value,
                    initialOffset: { ...offset.value }
                }
            }
        }

        // 优化后的缩放处理
        const handleTouchMove = (e: TouchEvent) => {
            e.preventDefault()
            if (!canvas.value) return

            // 单指拖拽
            if (e.touches.length === 1 && touchState.value.isDragging) {
                const touch = e.touches[0]
                offset.value = {
                    x: touchState.value.initialOffset.x + (touch.clientX - touchState.value.startX),
                    y: touchState.value.initialOffset.y + (touch.clientY - touchState.value.startY)
                }
                draw()
            }

            // 双指缩放（优化缩放速度和中心点）
            if (e.touches.length === 2) {
                const touch1 = e.touches[0]
                const touch2 = e.touches[1]

                // 计算中点
                const center = {
                    x: (touch1.clientX + touch2.clientX) / 2,
                    y: (touch1.clientY + touch2.clientY) / 2
                }

                // 计算缩放比例（添加缩放速度系数0.5）
                const distance = Math.hypot(
                    touch2.clientX - touch1.clientX,
                    touch2.clientY - touch1.clientY
                )
                const scaleFactor = 0.5 // 缩放速度系数
                const newScale = touchState.value.initialScale *
                    (1 + (distance - touchState.value.pinchDistance) * scaleFactor / touchState.value.pinchDistance)

                scale.value = newScale
                context.emit('scale', scale.value!);

                // 计算基于中点的偏移补偿
                const canvasPos = getCanvasPosition(center.x, center.y)
                offset.value = {
                    x: center.x - canvasPos.x * scale.value,
                    y: center.y - canvasPos.y * scale.value
                }

                draw()
            }
        }

        onMounted(() => {
            if (!canvas.value) return
            ctx.value = canvas.value.getContext('2d')
            canvas.value.width = window.innerWidth
            canvas.value.height = window.innerHeight
            if (props.initData.lines) {
                blocks.value = props.initData.blocks;
                lines.value = props.initData.lines;
            }
            image.src = props.imageUrl
            image.onload = () => {
                context.emit('scale', scale.value);
                draw();
            }
        })

        return {
            canvas,
            handleTouchStart,
            handleTouchMove,
            handleTouchEnd: () => {
                touchState.value.isDragging = false
                // selectedBlock.value = null // 重置选中状态
            },
            lastType
        }
    }
})
</script>