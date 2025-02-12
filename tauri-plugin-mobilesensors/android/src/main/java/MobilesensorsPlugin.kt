package com.plugin.mobilesensors

import android.app.Activity
import android.content.Context
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

class OrientationSensorManager(private val context: Context) : SensorEventListener {
    private var sensorManager: SensorManager? = null
    private var geomagneticRotationVectorSensor: Sensor? = null
    private var orientation: Float = 0.0f // 用于存储方位角

    init {
        sensorManager = context.getSystemService(Context.SENSOR_SERVICE) as SensorManager
        geomagneticRotationVectorSensor = sensorManager?.getDefaultSensor(Sensor.TYPE_GEOMAGNETIC_ROTATION_VECTOR)
    }

    fun startListening() {
        geomagneticRotationVectorSensor?.also {
            sensorManager?.registerListener(this, it, SensorManager.SENSOR_DELAY_NORMAL)
        }
    }

    fun stopListening() {
        sensorManager?.unregisterListener(this)
    }

    fun getOrientation(): Float {
        return orientation
    }

    override fun onSensorChanged(event: SensorEvent) {
        if (event.sensor.type == Sensor.TYPE_GEOMAGNETIC_ROTATION_VECTOR) {
            val rotationMatrix = FloatArray(9)
            SensorManager.getRotationMatrixFromVector(rotationMatrix, event.values)
            val orientationAngles = FloatArray(3)
            SensorManager.getOrientation(rotationMatrix, orientationAngles)
            orientation = Math.toDegrees(orientationAngles[0].toDouble()).toFloat()
            if (orientation < 0) {
                orientation += 360
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) {
        // 精度变化时的处理（可选实现）
    }
}


@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class MobilesensorsPlugin(private val activity: Activity) : Plugin(activity) {
    private lateinit var orientationSensorManager: OrientationSensorManager

    override fun load(webView: WebView) {
        super.load(webView)
        orientationSensorManager = OrientationSensorManager(activity)
        orientationSensorManager.startListening()
    }

    @Command
    fun getOrientation(invoke: Invoke) {
        val result = JSObject()
        result.put("orientation", orientationSensorManager.getOrientation())
        invoke.resolve(result)
    }
}