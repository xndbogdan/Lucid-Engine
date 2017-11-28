
package com.company;
import javafx.scene.media.Media;
import javafx.scene.media.MediaPlayer;
import com.sun.org.apache.xerces.internal.impl.xpath.XPath;

import java.awt.*;
import java.awt.image.BufferStrategy;
import java.awt.image.BufferedImage;
import java.awt.image.DataBufferInt;
import java.util.ArrayList;
import javax.swing.JFrame;
import javax.imageio.*;
import javax.sound.sampled.*;

public class Game extends JFrame implements Runnable{
    public class Bot implements Runnable
    {
        public Bot()
        {

        }
        public void run()//TODO - everything
        {
            while(true)
            {
                camera.forward=true;
                try
                {
                    Thread.sleep(10);
                }
                catch (Exception ex)
                {

                }

            }
        }

    }

    public class Shooter implements Runnable
    {
        public Shooter()
        {
        }
        public void run()
        {
            try {
                currentGun = ImageIO.read(currentGunFFile);
            }
            catch(Exception ex)
            {

            }
            Firing = false;

            try{
                AudioInputStream ais = AudioSystem.getAudioInputStream(new java.io.File(gunPath+"Gun"+selectedGun+".wav"));
                Clip test = AudioSystem.getClip();

                test.open(ais);
                test.start();
                Thread.sleep(200);
                try {
                    currentGun = ImageIO.read(currentGunFile);
                }
                catch(Exception ex)
                {

                }
                while (!test.isRunning())
                    Thread.sleep(10);
                while (test.isRunning())
                    Thread.sleep(10);

                test.close();



            }catch(Exception ex){
                ex.printStackTrace();
            }
        }
        public void updateGun(boolean Firing)
        {
            try
            {
                if(Firing)  currentGun = ImageIO.read(currentGunFFile);
                else
                {
                    while(lag<40)
                    {
                        Thread.sleep(40);
                        currentGun = ImageIO.read(currentGunFile);

                    }
                    lag=0;
                }

            }
            catch(Exception ex)
            {

            }
        }

    }
    public class Stepper implements Runnable
    {
        String location;
        public Stepper(String location)
        {
            this.location=location;
        }
        public void run()
        {
            if(!Stepping)
            {
                Stepping=true;
                //System.out.println("Step");
                try{
                    AudioInputStream ais = AudioSystem.getAudioInputStream(new java.io.File(location));
                    Clip test = AudioSystem.getClip();

                    test.open(ais);
                    test.start();

                    while (!test.isRunning())
                        Thread.sleep(10);
                    while (test.isRunning())
                        Thread.sleep(10);

                    test.close();
                    Stepping = false;
                }catch(Exception ex){
                    ex.printStackTrace();
                }
            }
        }

    }
    public class MusicPlayer implements Runnable
    {
        String location;

        public MusicPlayer(String location)
        {
            this.location=location;
        }
        public void run()
        {
            if(!musicPlaying)
            {
                musicPlaying=true;
                while(true)
                {
                    int i=0;
                    while(i<1)
                    {


                        try{
                            AudioInputStream ais = AudioSystem.getAudioInputStream(new java.io.File(location+"track"+i+".wav"));
                            Clip test = AudioSystem.getClip();
                            test.open(ais);
                            FloatControl gainControl =
                                    (FloatControl) test.getControl(FloatControl.Type.MASTER_GAIN);
                            gainControl.setValue(-12.0f); // Reduce volume by 5 decibels.
                            test.start();

                            while (!test.isRunning())
                                Thread.sleep(10);
                            while (test.isRunning())
                                Thread.sleep(10);

                            test.close();
                            musicPlaying = false;
                        }catch(Exception ex){
                            ex.printStackTrace();
                        }
                        i++;
                    }
                }


            }
        }

    }
    public static void changeGun(int index)
    {
        selectedGun = index;
        currentGunFile = new java.io.File(gunPath + "Gun" + index + ".png");
        currentGunFFile = new java.io.File(gunPath + "Gun" + index + "F.png");
        try
        {
            currentGun = ImageIO.read(currentGunFile);
        }
        catch(Exception ex)
        {

        }

    }
    public static boolean musicPlaying=false;
    public static boolean Stepping=false;
    public static boolean Firing =false;
    public static int Health=100;
    private static final long serialVersionUID = 1L;
    public int mapWidth = 15;
    public int mapHeight = 15;
    private Thread thread;
    public static int selectedGun=1;
    private boolean running;
    private BufferedImage image;
    public int[] pixels;
    public ArrayList<Texture> textures;
    public Camera camera;
    public Screen screen;
    public static java.awt.image.BufferedImage currentGun;
    public static java.io.File currentGunFile;
    public static java.io.File currentGunFFile;
    public static int[][] map;
    public Thread SoundThread;
    public static String gunPath="res/";
    SinWave gunWave;
    SinWave bobbingWave;
    public int lag=0;



    public Game(int Res_X, int Res_Y, int[][] map) {
        gunWave = new SinWave(0.6f,10,0.5f);
        gunWave.setX(0f);
        bobbingWave = new SinWave(1.2f,10,0.5f);
        bobbingWave.setX(0f);
        currentGunFile = new java.io.File(gunPath+"Gun"+ selectedGun +".png");
        currentGunFFile = new java.io.File(gunPath +"Gun" + selectedGun +"F.png");
        try
        {
            currentGun = ImageIO.read(currentGunFile);
        }
        catch(Exception ex)
        {

        }
        this.map = map;
        thread = new Thread(this);
        image = new BufferedImage(Res_X, Res_Y, BufferedImage.TYPE_INT_RGB);
        pixels = ((DataBufferInt)image.getRaster().getDataBuffer()).getData();
        textures = new ArrayList<Texture>();
        textures.add(Texture.redbrick);
        textures.add(Texture.brick);
        textures.add(Texture.greystone);
        textures.add(Texture.stone);
        screen = new Screen(map, mapWidth, mapHeight, textures, Res_X, Res_Y);
        camera = new Camera(4.5, 4.5, 1, 0, 0, -0.66);

        addKeyListener(camera);
        addMouseMotionListener(camera);
        addMouseListener(camera);
        setSize(Res_X, Res_Y);
        setResizable(false);
        setTitle("Lucid Frameworks");
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setBackground(Color.black);
        setLocationRelativeTo(null);
        setVisible(true);
        MusicPlayer a = new MusicPlayer("res/");
        SoundThread = new Thread(a);
        SoundThread.start();
        start();
    }
    private synchronized void start() {
        running = true;
        thread.start();
    }
    public synchronized void stop() {
        running = false;
        try {
            thread.join();
        } catch(InterruptedException e) {
            e.printStackTrace();
        }
    }



    public void render() {
        BufferStrategy bs = getBufferStrategy();
        if(bs == null) {
            createBufferStrategy(3);
            return;
        }

        Graphics g = bs.getDrawGraphics();

        g.setFont(new Font("TimesRoman", Font.PLAIN, 30));

        g.drawImage(image, 0, 0, image.getWidth(), image.getHeight(), null);
        //^ After gun draw hud
        /*
            for(int i=pixels.length/2+pixels.length/3; i<pixels.length; i++){
            pixels[i] = 299;

        }
         */
        //^Legacy code not used anymore
        g.setColor(Color.decode("299"));
        if(camera.forward || camera.back || camera.strafe_right || camera.strafe_left)
            g.drawImage(currentGun,0+image.getWidth()/2+image.getWidth()/8-(int)bobbingWave.getY()/4,0+image.getHeight()/3 + image.getHeight()/8-(int)gunWave.getY(),null);
        else
        {
            bobbingWave.setX(0);
            gunWave.setX(0);
            g.drawImage(currentGun,0+image.getWidth()/2+image.getWidth()/8,0+image.getHeight()/3 + image.getHeight()/8,null);
        }

        g.fillRect(0,image.getHeight()/2+image.getHeight()/3,image.getWidth(),image.getWidth());
        g.setColor(Color.ORANGE);
        g.drawString("Health: "+ Health,screen.width/12,screen.height-screen.height/12);
        g.drawLine(image.getWidth()/2-10,image.getHeight()/2,image.getWidth()/2-5,image.getHeight()/2);
        g.drawLine(image.getWidth()/2+10,image.getHeight()/2,image.getWidth()/2+5,image.getHeight()/2);
        g.drawLine(image.getWidth()/2,image.getHeight()/2-10,image.getWidth()/2,image.getHeight()/2-5);
        g.drawLine(image.getWidth()/2,image.getHeight()/2+10,image.getWidth()/2,image.getHeight()/2+5);
        /*
        Audio Part!
         */


        if(Firing)
        {
            Shooter a = new Shooter();
            SoundThread = new Thread(a);
            SoundThread.start();
        }

        g.setFont(new Font("TimesRoman", Font.PLAIN, 30));
        g.setColor(Color.ORANGE);

        g.drawString("Health: "+ Health,screen.width/12,screen.height-screen.height/12);

        bs.show();
        if(camera.forward || camera.back || camera.strafe_right || camera.strafe_left)
        {
            gunWave.update(0.08);
            bobbingWave.update(0.08);
            if((int)gunWave.getY()==-9)
            {

                Stepper a = new Stepper("res/Step.wav");
                SoundThread = new Thread(a);
                SoundThread.start();

            }
        }
        else gunWave.setX(0);
    }
    public void run() {
        long lastTime = System.nanoTime();
        final double ns = 1000000000.0 / 60.0;//60 times per second
        double delta = 0;
        requestFocus();
        while(running) {
            long now = System.nanoTime();
            delta = delta + ((now-lastTime) / ns);
            lastTime = now;
            while (delta >= 1)//Make sure update is only happening 60 times a second
            {
                //handles all of the logic restricted time
                screen.update(camera, pixels);
                camera.update(map);
                delta--;
            }
            render();//displays to the screen unrestricted time
        }
    }

}
