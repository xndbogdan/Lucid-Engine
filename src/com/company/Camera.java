package com.company;

import java.awt.*;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.awt.event.MouseEvent;
import java.awt.event.MouseListener;
import java.awt.event.MouseMotionListener;
import java.awt.Robot;

public class Camera implements KeyListener,MouseListener, MouseMotionListener{
    public double xPos, yPos, xDir, yDir, xPlane, yPlane;
    public boolean left, right, forward, back;
    public boolean strafe_left, strafe_right;
    public final double MOVE_SPEED = .06;
    public final double ROTATION_SPEED = .045;
    private int mouseX=0;

    Robot a;
    public Camera(double x, double y, double xd, double yd, double xp, double yp) {

        xPos = x;
        yPos = y;
        xDir = xd;
        yDir = yd;
        xPlane = xp;
        yPlane = yp;

        try
        {
            a = new Robot();
        }
        catch (Exception ex)
        {

        }



    }
    public void keyPressed(KeyEvent key) {
        if((key.getKeyCode() == KeyEvent.VK_LEFT))
            left = true;
        if((key.getKeyCode() == KeyEvent.VK_RIGHT))
            right = true;
        if((key.getKeyCode() == KeyEvent.VK_UP))
            forward = true;
        if((key.getKeyCode() == KeyEvent.VK_DOWN))
            back = true;
        if((key.getKeyCode() == KeyEvent.VK_A))
            strafe_left = true;
        if((key.getKeyCode() == KeyEvent.VK_D))
            strafe_right = true;
    }
    public void keyReleased(KeyEvent key) {
        if((key.getKeyCode() == KeyEvent.VK_LEFT))
            left = false;
        if((key.getKeyCode() == KeyEvent.VK_RIGHT))
            right = false;
        if((key.getKeyCode() == KeyEvent.VK_UP))
            forward = false;
        if((key.getKeyCode() == KeyEvent.VK_DOWN))
            back = false;
        if((key.getKeyCode() == KeyEvent.VK_A))
            strafe_left = false;
        if((key.getKeyCode() == KeyEvent.VK_D))
            strafe_right = false;
    }
    @Override
    public void mouseEntered(MouseEvent e)
    {

    }
    @Override
    public void mouseExited(MouseEvent e)
    {

    }
    @Override
    public void mouseClicked(MouseEvent e)
    {

    }
    @Override
    public void mousePressed(MouseEvent e)
    {

    }
    @Override
    public void mouseReleased(MouseEvent e)
    {

    }
    @Override
    public void mouseMoved(MouseEvent e)
    {
        /*
        if(mouseX>e.getX())
        {
            left=true;
        }
        else if(mouseX<e.getX())
        {
            right=true;
        }
        else
        {
            left=false;
            right=false;
        }
        mouseX=e.getX();
        */
    }
    @Override
    public void mouseDragged(MouseEvent e)
    {
    }

    public void update(int[][] map) {
        if(strafe_left)
        {
            if(map[(int)(xPos)][(int)(yPos+MOVE_SPEED*xDir)] == 0 && map[(int)(xPos-MOVE_SPEED*yDir)][(int)yPos] == 0 )
            {
                yPos+=MOVE_SPEED/1.7*xDir;
                xPos-=MOVE_SPEED/1.7*yDir;
            }

        }
        if(strafe_right)
        {
            if(map[(int)(xPos)][(int)(yPos-MOVE_SPEED*xDir)] == 0 && map[(int)(xPos+MOVE_SPEED*yDir)][(int)yPos] == 0 )
            {
                yPos-=MOVE_SPEED/1.7*xDir;
                xPos+=MOVE_SPEED/1.7*yDir;
            }

        }
        if(forward) {
            if(map[(int)(xPos + xDir * MOVE_SPEED)][(int)yPos] == 0) {
                xPos+=xDir*MOVE_SPEED;
            }
            if(map[(int)xPos][(int)(yPos + yDir * MOVE_SPEED)] ==0)
                yPos+=yDir*MOVE_SPEED;
        }
        if(back) {
            if(map[(int)(xPos - xDir * MOVE_SPEED)][(int)yPos] == 0)
                xPos-=xDir*MOVE_SPEED;
            if(map[(int)xPos][(int)(yPos - yDir * MOVE_SPEED)]==0)
                yPos-=yDir*MOVE_SPEED;
        }
        if(right) {
            double oldxDir=xDir;
            xDir=xDir*Math.cos(-ROTATION_SPEED) - yDir*Math.sin(-ROTATION_SPEED);
            yDir=oldxDir*Math.sin(-ROTATION_SPEED) + yDir*Math.cos(-ROTATION_SPEED);
            double oldxPlane = xPlane;
            xPlane=xPlane*Math.cos(-ROTATION_SPEED) - yPlane*Math.sin(-ROTATION_SPEED);
            yPlane=oldxPlane*Math.sin(-ROTATION_SPEED) + yPlane*Math.cos(-ROTATION_SPEED);
        }
        if(left) {
            double oldxDir=xDir;
            xDir=xDir*Math.cos(ROTATION_SPEED) - yDir*Math.sin(ROTATION_SPEED);
            yDir=oldxDir*Math.sin(ROTATION_SPEED) + yDir*Math.cos(ROTATION_SPEED);
            double oldxPlane = xPlane;
            xPlane=xPlane*Math.cos(ROTATION_SPEED) - yPlane*Math.sin(ROTATION_SPEED);
            yPlane=oldxPlane*Math.sin(ROTATION_SPEED) + yPlane*Math.cos(ROTATION_SPEED);
        }
    }
    public void keyTyped(KeyEvent arg0) {
        // TODO Auto-generated method stub

    }
}