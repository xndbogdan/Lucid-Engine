package com.company;

public class SinWave {
    private float x;
    private float y;
    private float s;
    private float a;
    private float f;

    public SinWave(float speed, float amplitude, float frequency){
        x = 0;
        y = 0;
        s = speed;
        a = amplitude;
        f = frequency;
    }

    public void update(double delta){
        x += s * delta;
        y = (float)(a * Math.sin(x * f));
    }

    public float getX() {
        return x;
    }
    public float getY() {
        return y;
    }
    public float getSpeed() {
        return s;
    }
    public float getAmplitude() {
        return a;
    }
    public float getFrequency() {
        return f;
    }

    public void setX(float x) {
        this.x = x;
    }
    public void setY(float y) {
        this.y = y;
    }
    public void setSpeed(float speed) {
        s = speed;
    }
    public void setAmplitude(float amplitude) {
        a = amplitude;
    }
    public void setFrequency(float frequency) {
        f = frequency;
    }
}
