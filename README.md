# Presentazioni
Un programmino per creare delle presentazioni carine a partire da un markdown. 
Credo di averne bisogno perché ho l'impressione che sarà necessario durante la
mia carriera e durante la mia vita creare diverse presentazioni. Per cui ho
bisogno di qualcosa che mi permetta di creare una presentazione carina a
partire da un template in modo rapido e semplice.  
Le presentazioni sono gestite in markdown. Infatti l'ispirazione di questo
programma viene dalle slides di obsidian. D'altra parte, ho notato che un mio
docente delle diapositive molto semplici e carina, che si spostano con lo stesso
keybinding di vim. Mi sembra che il programma per gestire le pagine web del mio
docente fosse scritto in java. Non ho investigato molto quest'ultimo, mi sono
solo ispirato all'interfaccia grafica. L'idea è unire le due cose (mi sembra che
anche slides di obsidian abbia i keybinding di vim, ma non è l'unica cosa che
copio dalle diapositive del mio professore).

## Funzionamento generale:
Il programma prende in input una cartella contenente file Markdown e li 
trasforma in pagine HTML. Ogni sezione Markdown del tipo header (indicata da #, 
##, ####, ecc.) diventa una diapositiva nella pagina HTML risultante. Il 
contenuto sotto ogni header costituisce il contenuto della diapositiva.

## Struttura delle diapositive:
- Ogni diapositiva ha uno spazio predefinito per il titolo e uno per il 
contenuto.
- Il contenuto viene ridimensionato utilizzando il CSS `scale` per adattarsi 
allo spazio dedicato.
- Le diapositive sono indicate da sottili barre verticali ai bordi della pagina 
HTML, che sono presenti o meno a seconda della presenza di diapositive 
nelle direzioni indicate.

## Navigazione tra le diapositive:
- La navigazione può avvenire tramite le barre verticali o tramite le vim 
motions (h, j, k, l).

## Finestra dei contenuti/commenti:
- Premendo "c", si apre la finestra dei contenuti/commenti.
- Questa sezione può contenere informazioni dettagliate, riferimenti o 
spiegazioni aggiuntive.
- I commenti iniziano con "---" e terminano con "---".
- Premendo nuovamente "c", la finestra si chiude.

## Importazione di file:
- Con il comando `\import[[path]]`, il testo "path" viene sostituito con il 
contenuto del file al percorso specificato (relativo alla cartella di input).

## Avvio del programma:
- Il programma si avvia eseguendo il suo eseguibile seguito da due flag:
  - `-s path/to/source/directory` specifica la cartella di input.
  - `-o path/to/output/directory` specifica la cartella di output.

## Suggerimento:
- È consigliabile copiare la cartella "assets" che contiene i file CSS e 
JavaScript necessari per rendere le pagine HTML dinamiche come descritto sopra.
