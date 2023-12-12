#include<ncurses.h>

WINDOW *create_newwin(int height, int width, int starty, int startx)
{
      WINDOW *local_win = newwin(height, width, starty, startx);
      box(local_win, 0, 0);
      wprintw(local_win, "CAN YOU SEE ME MOVE");
      wrefresh(local_win);
      return (local_win);
}

int main()
{
       int ch;
       int x= 10, y= 5, h=5, wid= 20;
       initscr();
       noecho();
       //curs_set(FALSE);
       keypad(stdscr, TRUE);
       cbreak();
       refresh();

       printw("PRINT q to EXIT\n");
       WINDOW* wn= create_newwin(h, wid, y, x);                      //tried it directly with newwin() but nothing

       while((ch=getch())!= 'q')                                      //function to move a window
       {
             switch(ch) 
             {
                    case KEY_LEFT: if(x>0)  --x;
                                    break;
                    case KEY_RIGHT: if(x<(COLS-16)) ++x;
                                    break;
                    case KEY_UP: if(y>0)    --y;
                                    break;
                    case KEY_DOWN: if(y<(LINES-6))  ++y;
                                    break;
            }
            mvprintw(0,0,"%d %d",y, x);

            mvwin(wn, y, x);
            touchwin(stdscr);
            // clear();
            refresh();
            wrefresh(wn);
    }

    delwin(wn);                                                     //clean up window
    endwin();
    return 0;
 }
