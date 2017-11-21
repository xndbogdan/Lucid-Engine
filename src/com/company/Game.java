package com.company;

import java.awt.*;
import java.awt.image.BufferStrategy;
import java.awt.image.BufferedImage;
import java.awt.image.DataBufferInt;
import java.util.ArrayList;
import javax.swing.JFrame;
import javax.imageio.*;
import javax.sound.sampled.*;

public class Game extends JFrame implements Runnable{
<<<<<<< HEAD
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
        String location;
        public Shooter(String location)
        {
            this.location=location;
        }
        public void run()
        {
            Firing = false;
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

            }catch(Exception ex){
                ex.printStackTrace();
            }



        }

    }
    public static boolean Firing =false;
=======
>>>>>>> 9142db8412ace0e6be47206d51b3523da0ff7d6a
    public static int Health=100;
    private static final long serialVersionUID = 1L;
    public int mapWidth = 15;
    public int mapHeight = 15;
    private Thread thread;
    private boolean running;
    private BufferedImage image;
    public int[] pixels;
    public ArrayList<Texture> textures;
    public Camera camera;
    public Screen screen;
<<<<<<< HEAD
    public java.awt.image.BufferedImage currentGun;
    public java.io.File currentGunFile;
    public java.io.File currentGunFFile;
    public static int[][] map;
    public Thread SoundThread;
    public int lag=0;
    public void updateGun(String location, boolean Firing)
    {
        try
        {
            if(Firing)  currentGun = ImageIO.read(currentGunFFile);
            else
            {
                if(lag>40)
                {
                    currentGun = ImageIO.read(currentGunFile);
                    lag=0;
                }
                else lag++;
            }

        }
        catch(Exception ex)
        {

        }
    }

    public Game(int Res_X, int Res_Y, int[][] map) {
        currentGunFile = new java.io.File("res/Gun1.png");
        currentGunFFile = new java.io.File("res/Gun1F.png");

=======
    public static int[][] map;

    public Game(int Res_X, int Res_Y, int[][] map) {
>>>>>>> 9142db8412ace0e6be47206d51b3523da0ff7d6a
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
<<<<<<< HEAD

        g.setFont(new Font("TimesRoman", Font.PLAIN, 30));

        g.drawImage(image, 0, 0, image.getWidth(), image.getHeight(), null);

        g.drawImage(currentGun,0+image.getWidth()/2+image.getWidth()/8,0+image.getHeight()/3 + image.getHeight()/8,null);


        //^ After gun draw hud
        /*
            for(int i=pixels.length/2+pixels.length/3; i<pixels.length; i++){
            pixels[i] = 299;

        }
         */
        //^Legacy code not used anymore
        g.setColor(Color.decode("299"));
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
        updateGun("res/Gun1.png",Firing);
        if(Firing)
        {
            Shooter a = new Shooter("res/Gun1.wav");
            SoundThread = new Thread(a);
            SoundThread.start();
        }

=======
        g.setFont(new Font("TimesRoman", Font.PLAIN, 30));
        g.setColor(Color.ORANGE);
        g.drawImage(image, 0, 0, image.getWidth(), image.getHeight(), null);
        g.drawString("Health: "+ Health,screen.width/12,screen.height-screen.height/12);
>>>>>>> 9142db8412ace0e6be47206d51b3523da0ff7d6a
        bs.show();
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
    public static void main(String [] args) {
        int[][] map =
                {
                        {1,1,1,1,1,1,1,1,2,2,2,2,2,2,2},
                        {1,0,0,0,0,0,0,0,2,0,0,0,0,0,2},
                        {1,0,3,3,3,3,3,0,0,0,0,0,0,0,2},
                        {1,0,3,0,0,0,3,0,2,0,0,0,0,0,2},
                        {1,0,3,0,0,0,3,0,2,2,2,0,2,2,2},
                        {1,0,3,0,0,0,3,0,2,0,0,0,0,0,2},
                        {1,0,3,3,0,3,3,0,2,0,0,0,0,0,2},
                        {1,0,0,0,0,0,0,0,2,0,0,0,0,0,2},
                        {1,1,1,1,1,1,1,1,4,4,4,0,4,4,4},
                        {1,0,0,0,0,0,1,4,0,0,0,0,0,0,4},
                        {1,0,0,0,0,0,1,4,0,0,0,0,0,0,4},
                        {1,0,0,0,0,0,1,4,0,3,3,3,3,0,4},
                        {1,0,0,0,0,0,1,4,0,3,3,3,3,0,4},
                        {1,0,0,0,0,0,0,0,0,0,0,0,0,0,4},
                        {1,1,1,1,1,1,1,4,4,4,4,4,4,4,4}
                };
<<<<<<< HEAD
        //^ The map will be loaded from file in the future
        Game game = new Game(1024,768,map);


=======
        //^ The map will be loaded in the future
        Game game = new Game(1024,768,map);
>>>>>>> 9142db8412ace0e6be47206d51b3523da0ff7d6a
    }
}
