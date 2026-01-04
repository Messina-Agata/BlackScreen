import tkinter as tk
from screeninfo import get_monitors

def crea_schermo(monitor):
    root = tk.Toplevel()
    root.configure(bg="black")
    # Posiziona la finestra esattamente sul monitor
    root.geometry(f"{monitor.width}x{monitor.height}+{monitor.x}+{monitor.y}")
    # Rimuove la barra superiore
    root.overrideredirect(True)
    # Porta la finestra in primo piano
    root.lift()
    root.focus_force()
    # Chiudi al click
    root.bind("<Button-1>", lambda e: main.destroy())
    root.bind("<Escape>", lambda e: main.destroy())
    return root

# finestra principale invisibile
main = tk.Tk()
main.withdraw()

# crea una finestra nera per ogni monitor
for m in get_monitors():
    crea_schermo(m)

main.mainloop()