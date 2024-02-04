# Presentazioni

Sto sviluppando un sito web, per fare delle presentazioni, tipo power point.  
Una presentazione è divisa in un titolo e nel contenuto. Di seguito il css.

```css
#slide {
/* ha due elementi: title e content */
}

#title {
/* ha un header. Uno di h1, h2, h3, h4, h5, h6 */
}

#content {
/* potrebbe contentere qualunque elemento html */
}
```

Puoi cambiare il css, in modo tale che:
- sia H l'altezza disponibile al container della diapositiva attuale.
- Sia W la larghezza disonibile al container della diapositiva attuale.
- Viene lasciato il 10% di H come margine superiore.
- Poi c'è il titolo.
- Il titolo può essere alto al massimo il 15% di H.
- C'è il 5% di H come margine tra il titolo e il contenuto.
- C'è il contenuto.
- C'è il 10% di margine verticale dalla fine della pagina.
- Il titolo e il contenuto lasciano il 10% di margine a destra
- Il titolo e il contenuto lasciano il 10% di margine a sinistra
- Il titolo ha dimensione massima
- Il contenuto ha dimensione massima
- Il titolo è centrato orizzontalmente.
- Il titolo è centrato verticalmente rispetto allo spazio che ha a disposizione.
- Il contenuto è centrato verticalmente rispetto allo spazio che ha a disposizione
- Il contenuto è allineato a sinistra.
