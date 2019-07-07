#[allow(clippy::unreadable_literal)]
pub const MINHASH_PERMUTATIONS: [[u64; 2]; 256] = [
    [853146490016488653, 1089606993368836715],
    [1849332765672628665, 726972438868274737],
    [1131688930666554379, 66204585613901025],
    [1936485333668353377, 1078410179646709132],
    [890837126813020267, 1343470117098523467],
    [1988249303247129861, 698653121981343911],
    [1408894512544874755, 1248486536592473639],
    [2140251716176616185, 1447963007834012793],
    [1755124413189049421, 1034598851883537815],
    [1355916793659431597, 1474008409379745934],
    [546586563822844083, 793773480906057541],
    [497603761441203021, 980501101461882479],
    [2000709902557454173, 963941556313537655],
    [1057597903350092207, 233651787311327325],
    [1576204252850880253, 243905121737149907],
    [2078784234495706739, 570269452476776142],
    [1022616668454863635, 297633284648631084],
    [2150082342606334489, 1516796967247398557],
    [712341150087765807, 1494795672066692649],
    [1511757510246096559, 1728741177365151059],
    [1525853819909660573, 1029197538967983408],
    [1263771796138990131, 1660732464170610344],
    [1215963627200985263, 1399769594446678069],
    [590069150281426443, 506465470557005705],
    [130824646248385081, 1279720146829545181],
    [962725325544728503, 860096419955634036],
    [1702561325943522847, 411519685280832908],
    [296074222435072629, 69539191273403207],
    [490211158716051523, 1960489729088056217],
    [1255327197241792767, 605092075716397684],
    [699458998727907367, 1017496016211653149],
    [32930168991409845, 1304834535101321372],
    [1985097843455124585, 949013511180032347],
    [362027841570125531, 1142776242221098779],
    [1903252144040897835, 576980004709031232],
    [900391845076405289, 1071272177143100544],
    [547470123601853551, 1494527341093835499],
    [1689373724032359119, 1073290814142727850],
    [845594231933442371, 1285904200674942617],
    [400331968021206285, 1277176606329477335],
    [174967108345233429, 343788427301735585],
    [876513700861085019, 2100915269685487331],
    [505848386844809885, 1227711252031557450],
    [1920468508342256199, 18593166391963377],
    [1292611725303815789, 2101884148332688233],
    [963317239501343903, 191808277534686888],
    [1730880032297268007, 2170124912729392024],
    [284614929850059717, 918430470748151293],
    [1185026248283273081, 1831024560113812361],
    [2167288823816985197, 1951365515851067694],
    [1214905315086686483, 744352348473654499],
    [1555253098157439857, 1921518311887826722],
    [1048013650291539723, 2020165648600700886],
    [1238618594841147605, 1764930142256726985],
    [1213502582686547311, 1903893374912839788],
    [286300733803129311, 1449378957774802122],
    [1250358511639043529, 1435825328374066345],
    [407534797452854371, 833197549717762813],
    [960869149538623787, 2238991044337210799],
    [1722699901467253087, 748955638857938366],
    [1325704236119824319, 1834583747494146901],
    [196979859428570839, 222012292803592982],
    [1669408735473259699, 901238460725547841],
    [781336617016068757, 1501611130776083278],
    [1098266067998399169, 252607439879480016],
    [1029096290873755209, 1830836920644382659],
    [812739846428483509, 1454586271201205945],
    [2169881403982893233, 701123326708091164],
    [1359607697113797935, 410071830926226192],
    [1091536557707382549, 1490336338171061046],
    [114681553932939819, 2275179250815266246],
    [1045116931081485463, 330649240126621873],
    [1213599796462591091, 2035365158881816524],
    [1442612926077060845, 1034355364158559000],
    [1756031352708963265, 1871075133574473361],
    [1464880946139292057, 1841358209015281278],
    [2250020708259933461, 239376785815016087],
    [724863464377112073, 882956145368658445],
    [69773460978034507, 1582909132181092862],
    [671396618153714907, 2048201890518748211],
    [1714581968555756963, 1945737706863748725],
    [1092741769522797967, 417364378594666442],
    [1429473482081523299, 440501836058396731],
    [202375920058644147, 1816096953371555263],
    [1448881484563716971, 355849732621157371],
    [1511928734060647251, 1539755653401730569],
    [66561619491616805, 1718080767548780670],
    [1370603980379370069, 1158735260507728929],
    [1494541323890039065, 135348932673001715],
    [1316312582643208067, 692994612266834808],
    [1960792302230354059, 513809688814427061],
    [1076562492117846339, 1094892392854250236],
    [920942498330268581, 917825197520768721],
    [784222790434217639, 42200068259224423],
    [2129498860209516943, 1243516270633398404],
    [275475632432179293, 741222018005723947],
    [2258845039726954909, 501312706344650233],
    [806499254784286719, 454954542714357873],
    [1937954399052022711, 1963133520760353190],
    [222001281118793629, 1726750538874739920],
    [1307623898419553715, 1745678868807221025],
    [596378382324432713, 95320895308197238],
    [1143053900166573071, 1900589223613132472],
    [1533598023462319541, 1988311725300002714],
    [23638791382176157, 1395514799898133478],
    [2219569576407027459, 464591125115301484],
    [71141773826556783, 2278263074080835102],
    [1818388640254809153, 2068542555708683121],
    [501536663370423415, 2084675879954203152],
    [419347422027964479, 480363784445985327],
    [529660076339540247, 1691942475524520892],
    [155541404377593545, 1521386688261351366],
    [693227156398074245, 1895741993383583176],
    [1213063041964914711, 1534259041796049575],
    [1843840504919232965, 668165748954715122],
    [1222766661707938725, 615111447467183312],
    [1122794988174825159, 922602432521167981],
    [418819557664227619, 1432122605161741889],
    [190426400552559033, 471810803924483143],
    [1966147745747045233, 1452369029097230462],
    [576008166811391959, 1423635693232729979],
    [534601397270543087, 526916313177348801],
    [90597453448583541, 1346919341313124631],
    [963196704215243767, 1611825149194211525],
    [1412384602117531587, 1172886288991537433],
    [1994868388707757991, 1741793070951343683],
    [677251789321463389, 1445881108867176875],
    [1440715575333055487, 22909053312763337],
    [1345976427394144055, 347195363635084995],
    [623223221977032189, 400638654727817116],
    [229973393774569323, 1927035341004003106],
    [1100924927981486349, 2160611223284354784],
    [1142011898554619157, 1326721901958911448],
    [930280150002399645, 1755751570149532202],
    [256376844288243073, 1211370058617372757],
    [667647064886375763, 282395776514445198],
    [887523374622233489, 1000876068192587000],
    [438738791387778287, 1809427504699261534],
    [2303322767409643853, 1098679211823433041],
    [774850677511300129, 1241765056385641522],
    [1303394264914287545, 1284755431339346903],
    [30800983110547319, 577367008031816745],
    [1274065095153849719, 2025829086559322592],
    [2226301266577380223, 681025040779950763],
    [2230003637567278053, 906275083377423089],
    [1940404327832495849, 2200094770514161145],
    [155232665988166157, 2081186345299296727],
    [1006460737351558209, 374831829849243822],
    [1454611184522745519, 780947004755503428],
    [1201238312584161365, 572852516826597254],
    [1460216712899454407, 520948816314912678],
    [1083430459766309055, 589284061440240932],
    [2194066157233035993, 1412046637307019781],
    [1555635357808583563, 922505384555302972],
    [19844541185119039, 1858010175263466792],
    [741253768828065649, 2282103190963219984],
    [1081212012956342415, 1040830645870571533],
    [692604529379956107, 1614336116059583266],
    [2249659420220267707, 2203438353246368047],
    [571689073293454359, 101042363635321963],
    [1519369915971317387, 987133582396089674],
    [1503498466256768565, 34003328920707476],
    [624831449073827911, 1406821564018748473],
    [523566524560730733, 2149206835176574302],
    [400051089267801269, 2027753677426137592],
    [1926883841634180865, 1092175325127958507],
    [2266664228510473829, 304798152056861662],
    [720489687421883995, 1772184317608454989],
    [1046160226273956435, 970337005471157286],
    [961792785029357631, 578934670696284826],
    [1079845340689437947, 2246802187380843162],
    [1578017948247993271, 822133722095030739],
    [530980257050460361, 1613745449226210684],
    [1146615300263939365, 1333208952156098572],
    [77805721769554747, 860660131426141556],
    [1814123549994096591, 669056521107507681],
    [1374741249445251559, 92708650362129618],
    [436105780130202935, 1346651453756436045],
    [267180305551533365, 1219254489936282127],
    [675936151654593631, 1587863486233754297],
    [989447341617279643, 1007111213472533853],
    [2238684158818820039, 1732148872681962166],
    [1331764214305418433, 1781852975639818553],
    [598809232779830627, 33929809895439103],
    [1530704471618857191, 2176847430331439517],
    [560969485294346585, 2232484045903276723],
    [553511112399249535, 1136892702219427415],
    [1137355176578183129, 2054202913241361058],
    [1389417619759458077, 1357435380345254312],
    [1963554513091177761, 2301072449958134130],
    [1435961994841646313, 1327054849089190233],
    [476883306901073571, 1016640105897699543],
    [1075203252082498925, 211174321718731811],
    [1623230145688621745, 1300924433422090042],
    [1108349714537262649, 2200276897035118696],
    [2139930061752271683, 1645105813894706674],
    [1011184464903726013, 1181515140629003486],
    [1858604191563995829, 967815337348467244],
    [1210674227935444255, 217533146659530837],
    [1568127022686944529, 2124325251355125800],
    [387522773954877337, 205422378336623413],
    [1517888610267434301, 1953522170573947113],
    [700862400191169363, 1344552490737774094],
    [967097615699760379, 372059812979546869],
    [2262493808499142063, 14879511499043046],
    [1090444553077473263, 494437689008248400],
    [1582707150152068285, 1818916274072290591],
    [2054916210223584085, 2215414844390388342],
    [614072241193993649, 2255736517977871755],
    [319471570941800695, 221425464440391675],
    [2134889262352799711, 1238037478679691122],
    [1908720197982149627, 1611953700583020032],
    [1706543409596971629, 1149648113078654923],
    [1123377802309478593, 2065480490659500424],
    [234248550434195227, 838916658758481218],
    [129867854034951915, 2034568773939092299],
    [1132173899172731285, 1585274956277924856],
    [747397599768216393, 1000249192062601970],
    [966118328285669639, 890393120791290392],
    [1978744473069861947, 1397830565231711853],
    [1367649354516096993, 466350397001674941],
    [1888859023181744067, 1162881924911396581],
    [816698952722704181, 234300191628743592],
    [2064623720919573, 889304434558041494],
    [813333883477635751, 491451245247792106],
    [341622436532982245, 1363358953271550641],
    [1377838091044673, 2091031550191230350],
    [1225049478976914697, 1181152476972725364],
    [1015972483717875537, 444687044135710966],
    [2078032864408579469, 1634964244097888403],
    [1682213591465198327, 1035226665694375798],
    [412305785740128109, 1012868617861784491],
    [166693608092683829, 623582405922007707],
    [142001737827086217, 1373681019083513008],
    [1749349945778548429, 1412245043370335664],
    [249487271773300899, 748034660340058220],
    [2284010474415810475, 89587669385366596],
    [2116614295071563543, 896538142042373209],
    [982081696050850971, 238384078753871766],
    [1407517150232293031, 1227918329315752077],
    [1781334002448441555, 1133783862242472534],
    [555204413597398613, 156835912556655276],
    [2130854063125005169, 1620084504055877943],
    [1533769449207033351, 531348171498495190],
    [343511759321043983, 19020177087633722],
    [1096846909243812591, 1367659725005456180],
    [219853968883601293, 1246568561362943234],
    [1623242076811302017, 1270405659154214890],
    [1670457544761436411, 63913058899124031],
    [1510538207643510749, 316314260758899106],
    [755178036278346005, 1607298207013906952],
    [1758593919863430495, 910218677289284479],
    [1798669847980462979, 1377363600250827915],
    [1184950555241875363, 1034879193665968156],
    [949293229786807261, 1390525603722466192],
    [577374388210505249, 1452060099744603134],
];

#[allow(clippy::unreadable_literal)]
pub const CHUNKING_GEAR: [u64; 256] = [
    9584138480181866666,
    4739450037122062430,
    1042006760432515769,
    10675154520554330663,
    15869016765101259526,
    8970928072383595559,
    1399451202205921674,
    14523822808097149755,
    16268498464839721299,
    10481172452375523505,
    17104617054662428007,
    1589812074021361642,
    5529368114994898429,
    16097147859444922117,
    7366391750793198740,
    11100538009918328137,
    1389689728615383157,
    4977138822009172500,
    908349889557194910,
    14452518814433479233,
    2122926032271239532,
    591612022955043504,
    9379034436570273189,
    12748258297147873806,
    4307386326245858243,
    13845229916084989633,
    11224472648935237303,
    7047696390035316099,
    2021133566789993437,
    17387162748083618158,
    11746787256992261957,
    6644482612611712714,
    15729398955930993486,
    18187694890389888249,
    13375007170405426180,
    4646676434852504131,
    13152698236329639071,
    899989819383117385,
    1604228284900755822,
    13429168974601667864,
    3706248770764044735,
    3719799868214789934,
    339511817415309475,
    12306710798301877171,
    9844020938499650522,
    13507342816267977422,
    15331217600725578556,
    7506003564454403634,
    17943236144189306428,
    282153689319390566,
    7654271695669749695,
    2650412143911437370,
    6193440044944269691,
    9296646612477743744,
    15077579129862372948,
    67630558006200567,
    11937031764123301943,
    1634327986517329169,
    16073934395340319514,
    11660580892053471307,
    12301495579660351243,
    16908718276972184511,
    6851717516129410187,
    13288278789994352315,
    17482170774163197685,
    12177168157992128323,
    1679876621412537528,
    15666827561093998679,
    4235032027386979601,
    17396011814487376094,
    2036017399572567727,
    4977152437582070133,
    11341111713611820820,
    5866443846249079891,
    5131277185090952872,
    8325299058005558320,
    5701450024662049407,
    15870252139465586153,
    641910037851244477,
    5172232175829573378,
    2261684586607900474,
    11396825283718526131,
    12408680075109652465,
    7761877592432080901,
    13820035802684848169,
    8150091535052795450,
    1103357817677537274,
    13470426615970288837,
    4696524065622673976,
    9336804607285957500,
    13043178028673218162,
    7139020806469476608,
    12450708403507569100,
    2877039905016676547,
    15118872351294838361,
    3277072151995360446,
    1979210712452295885,
    14822651643543876641,
    5849754172112174627,
    13664543478254756807,
    16186972696580520130,
    14259131679517995788,
    1772106294408535188,
    2668205339646827112,
    3734021086026184498,
    4257506854909152229,
    6797729639474582495,
    3708095106171770747,
    15445894064208319783,
    11045733249000282278,
    6925260395759991481,
    6761677416581440942,
    3134957115005596133,
    5496794829211694837,
    225035875953155227,
    18051382753002575119,
    6911658830635795092,
    6648838042848840266,
    7680838377178993211,
    14373546918520540763,
    7385952462173201391,
    7500965322394952100,
    15539214383494689771,
    14355530880918970074,
    4040759991734970063,
    1335151750647325670,
    13713452291232361388,
    8852782707920062625,
    6076783566257059794,
    14451547968886132839,
    6756882940270420653,
    17423128808598833972,
    5877907771709558759,
    14308413074787508328,
    12294727846616188882,
    13766545313722789196,
    7000331838802888702,
    15110028412924060381,
    15869145452552081798,
    10836437530623796047,
    1273143868608979117,
    17728019699248776702,
    379008101491021165,
    6658832383485441856,
    6005905363267598720,
    4792802520786808134,
    17024928019214694263,
    7949301678895773307,
    14602122883430422290,
    6416689239839102410,
    18112987618441438141,
    5424513836620859057,
    12327961344656070412,
    18229731317766561349,
    6214341855555485197,
    14659604854593022088,
    18341976098904231516,
    9093141550798891276,
    4487469223051523007,
    12576621890114680116,
    11368566035561888278,
    16632902625329423294,
    13764076000271015053,
    11494903226088746337,
    14079100963083335535,
    5976601008655555884,
    5685807667042201553,
    16503266544486236927,
    5505089898459277917,
    17076606531971661551,
    939769563919939433,
    17217248958964594832,
    11196454443995107214,
    13253314556391295544,
    17340262486782904124,
    5483165811177129540,
    121736889831618943,
    6318157315988658220,
    14520375112718267902,
    689388276875596813,
    5273319774965020902,
    7975410517565653865,
    13935269057627157047,
    16821796908479891795,
    5882048506860913277,
    18003709489856105216,
    1424933842252756366,
    6634557257081066175,
    16179356916240399588,
    11153419399622634817,
    15654294493035402949,
    2652919763627807814,
    16437183290373292867,
    16903315446495122175,
    3575318971059548300,
    3073697257555445515,
    16187136733800880291,
    15191964085364171996,
    11982016174040399757,
    1948589207658719032,
    14444449012119241408,
    7130754012353479650,
    7480280819583944745,
    3603028513293740433,
    7021162527209392860,
    2124450348946366496,
    14349140477237426219,
    7396225914272122063,
    16288120608246645021,
    7309794834881975478,
    16746864570463829614,
    9239996606832866982,
    14126189643057989505,
    5785181374404079776,
    16681042508550037223,
    9085478584447523753,
    12879577862603639783,
    13351556131001260565,
    10860701565908202403,
    9109516948909639475,
    2942389181877553466,
    1907923359833671766,
    1700327967934711796,
    4355952370607563279,
    6159416062364401684,
    8120694842642123744,
    4670360822544180192,
    12684384265447906291,
    11518186189217338692,
    14839496566538901930,
    13515715604989800698,
    12135065096961528408,
    9056982071865174221,
    12690699907549395246,
    2080896935929507230,
    14546126411900211421,
    6222235617711806766,
    13387691023848518640,
    1259523422199249803,
    1733690531272524911,
    16691543548458831721,
    3252085970219428027,
    790320086519395195,
    8366099548552136926,
    357423734596052102,
    6375583027298966643,
    88639135753272123,
    13813972796887520980,
    8203570281250814300,
    18377325011640278855,
    2922465295015278442,
    2164203008979443347,
    7447171935848155518,
    3663261456454345351,
    5865411828910435346,
    13570376904595974307,
];
