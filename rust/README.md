# A simple genetic algorithm written in Rust

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/ga`
[0]: "SbedbcelXWRGhONyOFfiJugzbdKZGBqytGcNlaRuYWVO", fitness 0.13636363636363635
[1]: "SbedbcelXWRGhONyOFfiJugzbdKZGBqytGcNlaRuYWVO", fitness 0.13636363636363635
[2]: "SbedbcelXWRGhONyOFfiJugzbdKZGBqytGcNlaRuYWVO", fitness 0.13636363636363635
[3]: "TSbFuSoekhhruyZyTpnLmueTbQvCsamsKWEovmvyUdo ", fitness 0.1590909090909091
[4]: "TSbFuSoekhhruyZyTpnLmueTbQvCsamsKWEovmvyUdo ", fitness 0.1590909090909091
[5]: "TSbFuSoekhhruyZyTpnLmueTbQvCsamsKWEovmvyUdo ", fitness 0.1590909090909091
[6]: "EzdMIuUnerXDccDKfbSrouHpNtCyvIcGtcQEbelyOZSa", fitness 0.1590909090909091
[7]: "TSbFuSoekhhruyZyTpnLmueTbQvCsamsKWEovmvyUdo ", fitness 0.1590909090909091
[8]: "dreQYwPYOXbTYQhUibxzmFppsOWmfebvQJEovmvyUdo ", fitness 0.18181818181818182
[9]: "RreQYyiUrIUZSNq faT xaFpNiQpWzU mRflSJtT NKw", fitness 0.18181818181818182
[10]: "tBeB hleWZyqowPLLTkviuM JEzoFehJqKOVnRVyUdo ", fitness 0.20454545454545456
[11]: "tBeB hleWZyqowPLLTkviuM JEzoFehJqKOVnRVyUdo ", fitness 0.20454545454545456
[12]: "TSbFuSoekhhruyq fasviuM JEzoFebvQJEovmvyUdoC", fitness 0.25
[13]: "TSbFuSoekhhruyq fasviuM JEzoFebvQJEovmvyUdoC", fitness 0.25
[14]: "ssBeLFiUrIUZSNq foVKouHpLd tiqrpvDeIHlYmhdCu", fitness 0.25
[15]: "BVeXLAacdklThQp eoa eBmpsOWmfeHytGcVdRVyUdo ", fitness 0.2727272727272727
[16]: "TSbFuSoekhhruyZ foVKouHpLd tiqrpvDeIHlYmhdCu", fitness 0.29545454545454547
[17]: "TSbFuSoekhhruyZ foVKouHpLd tiqrpvDeIHlYmhdCu", fitness 0.29545454545454547
[18]: "dhhWKmwkZ vTowP foVKOueTJEzoKeSVAZI EyYs koj", fitness 0.29545454545454547
[19]: "TSbFuSoekhhruyZ foVKouHpLd tiqrpvDeIHBGfsdYg", fitness 0.3181818181818182
[20]: "TSbFuSoekhhruyZ foVKouHpLd tiqrpxoI lazUvMKw", fitness 0.3409090909090909
[21]: "TSbFuSoekhhruyZ foVKouHpLd tiqrpxoI lazUvMKw", fitness 0.3409090909090909
[22]: "TSbFuSoekhhruQp foVKouHpLd tiqrptcQEbmvyUdo ", fitness 0.3409090909090909
[23]: "RhhtFiGckhhruyZ foVKouHpLd tiqrytGcNlaRyUdo ", fitness 0.4090909090909091
[24]: "RhhtFiGckhhruyZ foVKouHpLd tiqrytGcNlaRyUdo ", fitness 0.4090909090909091
[25]: "RhhtFiGckhhruyZ foVKouHpLd tiqrytGcNlaRyUdo ", fitness 0.4090909090909091
[26]: "TVeXLAacd vTSNq foxzjuHpNtCoFeHytGczlak gdFg", fitness 0.4090909090909091
[27]: "dhhtFiGckhhruyZ foVKoumpsOWmfqrpAZI lazyUdow", fitness 0.4090909090909091
[28]: "drePquwcVxbfoRp fomwjsDMedzoFehJxMIJla yUdo ", fitness 0.4318181818181818
[29]: "dhhWRuicVxbfoyq fjhAjugzedzoFeb mRfJla yUdog", fitness 0.4772727272727273
[30]: "dhhWRuicVxbfoyq fjhAjugzedzoFeb mRfJla yUdog", fitness 0.4772727272727273
[31]: "dhhWRuicVxbfoyq fjhAjugzedzoFeb mRfJla yUdog", fitness 0.4772727272727273
[32]: "TVeXLAacd vTowq foxziuM ed tiqrpAZI lazyUdo ", fitness 0.5
[33]: "TVeXLAacd vTowq foxziuM ed tiqrpAZI lazyUdo ", fitness 0.5
[34]: "TVeXLAacd vTowq foxziuM ed tiqrpAZI lazyUdo ", fitness 0.5
[35]: "TVeXLAacd vGXZq faxKoumpedzoFeb mZI lazyUdo ", fitness 0.5
[36]: "TVeXLAacd vTowq foxziBmped tiqrpAZI lazyUdo ", fitness 0.5227272727272727
[37]: "Zha jhacd vTowP foxzouHpLd oFehytGc lazyUdo ", fitness 0.5454545454545454
[38]: "Zha jhacd vTowP foVKoumpedzoFeb mZI lazyUdo ", fitness 0.5454545454545454
[39]: "TVeXLAacZ vTowq foxVjuHpLd oFehytGc la yUdog", fitness 0.5681818181818182
[40]: "TVeXLAack vTowP foxzouHpLd oFehytGc lazyUdo ", fitness 0.5681818181818182
[41]: "TVeXLAack vTowP foxzouHpLd oFehytGc lazyUdo ", fitness 0.5681818181818182
[42]: "Zha jhGjkUZrowq foxziumzed fvBrGtheNlazyUdo ", fitness 0.5909090909090909
[43]: "Zha tuicVegTowq foxVjujledzoFeb AZe lazyUdo ", fitness 0.5909090909090909
[44]: "TSb tuicVxbfowk foxziumpLd oFebvtGE lazyUdo ", fitness 0.6136363636363636
[45]: "Tbe tuicd vTowk foxVjGjledzoFeb AZe lazyUdog", fitness 0.6363636363636364
[46]: "Tbe tuicd vTowk foxVjGjledzoFeb AZe lazyUdog", fitness 0.6363636363636364
[47]: "yTedbuickUZrowq foxwjuHpLd ovBrGtZI lazyUdow", fitness 0.6363636363636364
[48]: "Tbe tuicd vKowq foxzjuHpLd oFerpAZI lazy do ", fitness 0.6590909090909091
[49]: "TVe tuickUZrowq foxziBmpLd ofer tGc la yUdog", fitness 0.6818181818181818
[50]: "drePquicV browP foVVjympsd oFehytDe lazyUdog", fitness 0.6818181818181818
[51]: "Tbe tuicd vKowq foxzjuHped oFehytGe lazyUdog", fitness 0.7045454545454546
[52]: "TSb tuickhRrowQ foxwoumped oFehytZI lazy dog", fitness 0.7045454545454546
[53]: "Tbe tuicd bTowq foVKoumped oFehytGe lazyUdog", fitness 0.7045454545454546
[54]: "TVedbuickUZrowq foxziumpedzoFeb the lazyUdog", fitness 0.7272727272727273
[55]: "TVedbuickUZrowq foxziumpedzoFeb the lazyUdog", fitness 0.7272727272727273
[56]: "Tha tuicV browq foxzjuHped oFehytGe lazyUdo ", fitness 0.7272727272727273
[57]: "Tbe tuickhbruwn foxziumped oFehytGe lazyUdog", fitness 0.75
[58]: "TSb tuickhhrown foxKoumped oFeb the lazyUdog", fitness 0.7727272727272727
[59]: "TSb tuickhhrown foxKoumped oFeb the lazyUdog", fitness 0.7727272727272727
[60]: "TSb tuickhhrown foxKoumped oFeb the lazyUdog", fitness 0.7727272727272727
[61]: "Tha tuick biowq foxwjumped oFer AZe lazyUdog", fitness 0.7954545454545454
[62]: "Tbe tuick hrowk foxVoumped oFeb the lazyUdog", fitness 0.7954545454545454
[63]: "Thh tuicd browq foxzjumped oFerGtheNlazy dog", fitness 0.8181818181818182
[64]: "Thh tuicd browq foxzjumped oFerGtheNlazy dog", fitness 0.8181818181818182
[65]: "Tha tuick browq foxVjumpedzoFeb the lazyUdog", fitness 0.8181818181818182
[66]: "ThePquicV browP foxwjumped oFer tGe lazyUdog", fitness 0.8409090909090909
[67]: "Tha tuick browq foxzjumped oFeb the lazyUdog", fitness 0.8409090909090909
[68]: "ThePquicV brown foxKoumped oFeb the lazy dog", fitness 0.8636363636363636
[69]: "ThePquicV brown foxKoumped oFeb the lazy dog", fitness 0.8636363636363636
[70]: "ThePquicV brown foxKoumped oFeb the lazy dog", fitness 0.8636363636363636
[71]: "ThePquicV brown foxzjumped oFeb the lazyUdog", fitness 0.8636363636363636
[72]: "Tbe tuick browq foxzjumped ofer the lazy dog", fitness 0.8863636363636364
[73]: "Tbe tuick browq foxzjumped ofer the lazy dog", fitness 0.8863636363636364
[74]: "The tuickhbrown foxzjumped oFer the lazy dog", fitness 0.9090909090909091
[75]: "The quicV brown foxzjumped oFer the lazy dog", fitness 0.9318181818181818
[76]: "The quicV brown foxzjumped oFer the lazy dog", fitness 0.9318181818181818
[77]: "The quicV brown foxzjumped oFer the lazy dog", fitness 0.9318181818181818
[78]: "The quicV brown foxzjumped oFer the lazy dog", fitness 0.9318181818181818
[79]: "The quicV brown foxzjumped oFer the lazy dog", fitness 0.9318181818181818
[80]: "The tuick brown foxVjumped oFer the lazy dog", fitness 0.9318181818181818
[81]: "The tuick browq foxzjumped over the lazy dog", fitness 0.9318181818181818
[82]: "The quick brown foxzjumped over tGe lazy dog", fitness 0.9545454545454546
[83]: "The quick brown foxzjumped over tGe lazy dog", fitness 0.9545454545454546
[84]: "The quick browq foxwjumped over the lazy dog", fitness 0.9545454545454546
[85]: "The quick brown foxzjumped over tGe lazy dog", fitness 0.9545454545454546
[86]: "The quick brown foxzjumped over tGe lazy dog", fitness 0.9545454545454546
[87]: "ThePquick brown foxzjumped over the lazy dog", fitness 0.9545454545454546
[88]: "The quick brown foxVjumped over the lazy dog", fitness 0.9772727272727273
[89]: "The quick brown foxwjumped over the lazy dog", fitness 0.9772727272727273
[90]: "The quick brown foxwjumped over the lazy dog", fitness 0.9772727272727273
[91]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[92]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[93]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[94]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[95]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[96]: "The quick brown foxwjumped over the lazy dog", fitness 0.9772727272727273
[97]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[98]: "The quick brown foxzjumped over the lazy dog", fitness 0.9772727272727273
[99]: "The quick brown fox jumped over the lazy dog", fitness 1.0
```

## License

If this is worth using, modifying, or sharing, then feel free to do so.