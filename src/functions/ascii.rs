/* Copyright (C) 2026  limforge <limforge@neutronen.net>, justpav05


This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

// Создаем список ОС
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distro {
    /* Windows, MacOS */ Ubuntu,
    Arch,
    Fedora,
    Debian,
    Gentoo,
    CachyOS,
    EndeavourOS,
    Trisquel,
    NixOS,
    Bazzite,
    Manjaro,
    Artix,
    Void,
    ALT,
    Guix,
    Kali,
    OpenSUSE,
    Lubuntu,
    Xubuntu,
    Vanilla,
    Garuda,
    Deepin,
    Nobara,
    Tails,
    RedHat,
    Calculate,
    Devuan,
    CentOS,
    ElementaryOS,
    PopOS,
    FreeBSD,
    NetBSD,
    OpenBSD,
    GNU,
    Tux,
    Slackware,
    Zorin,
    Alpine,
    LinuxMint,
    Rocky,
    Solus,
    SteamOS,
    KdeNeon,
    Parrot,
    ArcoLinux,
    Asahi,
    MXLinux,
    Unknown,
}

impl Distro {
    fn normalize_key(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect()
    }

    pub fn from_string(name: &str) -> Self {
        let norm = Self::normalize_key(name);

        if norm == "void" || name.to_lowercase().contains("voidlinux") {
            return Distro::Void;
        }

        const REMOVED: &[&str] = &["omarchy", "qubes", "pikaos", "ultramarine", "oracle"];
        if REMOVED.contains(&norm.as_str()) {
            return Distro::Unknown;
        }

        // Exact aliases for --logo=Name (avoids false positives from substring rules)
        const EXACT: &[(&str, Distro)] = &[
            ("alpine", Distro::Alpine),
            ("linuxmint", Distro::LinuxMint),
            ("kdeneon", Distro::KdeNeon),
            ("steamos", Distro::SteamOS),
            ("holoiso", Distro::SteamOS),
            ("rocky", Distro::Rocky),
            ("almalinux", Distro::Rocky),
            ("mxlinux", Distro::MXLinux),
            ("arcolinux", Distro::ArcoLinux),
            ("asahi", Distro::Asahi),
            ("solus", Distro::Solus),
            ("parrot", Distro::Parrot),
            ("devuan", Distro::Devuan),
            ("cachyos", Distro::CachyOS),
            ("endeavouros", Distro::EndeavourOS),
            ("garuda", Distro::Garuda),
            ("artix", Distro::Artix),
            ("manjaro", Distro::Manjaro),
            ("nixos", Distro::NixOS),
            ("bazzite", Distro::Bazzite),
            ("nobara", Distro::Nobara),
            ("popos", Distro::PopOS),
            ("zorin", Distro::Zorin),
            ("gentoo", Distro::Gentoo),
            ("arch", Distro::Arch),
            ("fedora", Distro::Fedora),
            ("debian", Distro::Debian),
            ("ubuntu", Distro::Ubuntu),
            ("lubuntu", Distro::Lubuntu),
            ("xubuntu", Distro::Xubuntu),
            ("opensuse", Distro::OpenSUSE),
            ("slackware", Distro::Slackware),
            ("freebsd", Distro::FreeBSD),
            ("netbsd", Distro::NetBSD),
            ("openbsd", Distro::OpenBSD),
            ("tux", Distro::Tux),
        ];
        if let Some((_, d)) = EXACT.iter().find(|(k, _)| *k == norm) {
            return *d;
        }

        let name = name.to_lowercase();

        const RULES: &[(&str, Distro)] = &[
            //           ("windows", Distro::Windows),
            //           ("darwin", Distro::MacOS),
            //           ("macos", Distro::MacOS),
            ("linux mint", Distro::LinuxMint),
            ("linuxmint", Distro::LinuxMint),
            ("kde neon", Distro::KdeNeon),
            ("kdeneon", Distro::KdeNeon),
            ("steam os", Distro::SteamOS),
            ("steamos", Distro::SteamOS),
            ("holoiso", Distro::SteamOS),
            ("rocky", Distro::Rocky),
            ("almalinux", Distro::Rocky),
            ("alma linux", Distro::Rocky),
            ("mx linux", Distro::MXLinux),
            ("mxlinux", Distro::MXLinux),
            ("arcolinux", Distro::ArcoLinux),
            ("arco linux", Distro::ArcoLinux),
            ("asahi", Distro::Asahi),
            ("alpine", Distro::Alpine),
            ("solus", Distro::Solus),
            ("parrot", Distro::Parrot),
            ("lubuntu", Distro::Lubuntu),
            ("xubuntu", Distro::Xubuntu),
            ("ubuntu", Distro::Ubuntu),
            ("arch", Distro::Arch),
            ("fedora", Distro::Fedora),
            ("debian", Distro::Debian),
            ("gentoo", Distro::Gentoo),
            ("cachyos", Distro::CachyOS),
            ("endeavouros", Distro::EndeavourOS),
            ("trisquel", Distro::Trisquel),
            ("nixos", Distro::NixOS),
            ("bazzite", Distro::Bazzite),
            ("manjaro", Distro::Manjaro),
            ("artix", Distro::Artix),
            ("alt linux", Distro::ALT),
            ("guix", Distro::Guix),
            ("kali", Distro::Kali),
            ("opensuse", Distro::OpenSUSE),
            ("vanilla", Distro::Vanilla),
            ("garuda", Distro::Garuda),
            ("deepin", Distro::Deepin),
            ("nobara", Distro::Nobara),
            ("tails", Distro::Tails),
            ("rhel", Distro::RedHat),
            ("red hat", Distro::RedHat),
            ("calculate", Distro::Calculate),
            ("devuan", Distro::Devuan),
            ("centos", Distro::CentOS),
            ("elementary", Distro::ElementaryOS),
            ("pop_os", Distro::PopOS),
            ("popos", Distro::PopOS),
            ("pop os", Distro::PopOS),
            ("freebsd", Distro::FreeBSD),
            ("netbsd", Distro::NetBSD),
            ("openbsd", Distro::OpenBSD),
            ("tux", Distro::Tux),
            ("slackware", Distro::Slackware),
            ("zorin", Distro::Zorin),
        ];

        if norm == "gnu"
            || name.contains("gnu/linux")
            || name.contains("gnu linux")
        {
            return Distro::GNU;
        }

        // Longest keyword first so "linux mint" wins over "linux", "alpine" over "alt"
        let mut rules: Vec<(&str, Distro)> = RULES.to_vec();
        rules.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        rules
            .iter()
            .find(|(key, _)| name.contains(key))
            .map(|(_, distro)| *distro)
            .unwrap_or(Distro::Unknown)
    }
    // Возвращаем ASCII арт для каждого дистрибутива
    pub fn ascii_art(&self) -> String {
        let art = match self {
            /* Distro::Windows => r#"/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////

/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////
/////////////////  /////////////////"#,
                                    Distro::MacOS => r#"{G}        .:'
    __ :'__{RESET}
{Y} .'`  `-'  ``.\{RESET}
{O}:          .-'{RESET}
{R}:         :{RESET}
{M}:         `-;:{RESET}
{B}  `.__.-.__.'{RESET}"#, */
            Distro::Ubuntu => {
                r#"
{O}       ..;,; .,;,.{RESET}
{O}    .,lool: .ooooo,{RESET}
{O}   ;oo;:    .coool.{RESET}
{O} ....         ''' ,l;{RESET}
{O}:oooo,            'oo.{RESET}
{O}looooc            :oo'{RESET}
{O} '::'             ,oo:{RESET}
{O}   ,.,       .... co,{RESET}
{O}    lo:;.   :oooo; .{RESET}
{O}     ':ooo; cooooc{RESET}
{O}        '''  ''''{RESET}"#
            }

            Distro::Arch => {
                r#"
{M}         /\{RESET}
{M}        /  \{RESET}
{M}       /    \{RESET}
{B}      /      \{RESET}
{B}     /   ,,   \{RESET}
{B}    /   |  |   \{RESET}
{B}   /_-''    ''-_\{RESET}"#
            }
            Distro::Fedora => {
                r#"

{B}     __{RESET}
    /  \{RESET}
{B} __ |_{RESET}
{B}/   |{RESET}
{B}\__/{RESET}"#
            }
            Distro::Gentoo => {
                r#"
{M} _-----_{RESET}
{M}(       \{RESET}
{M}\    0   \{RESET}
{M}\        ){RESET}
{M} /      _/{RESET}
{M}(     _-{RESET}
{M}\____-{RESET}"#
            }
            Distro::CachyOS => {
                r#"
{G}    ____________{RESET}
{G}   /            /  o{RESET}
{G}  /      ______/{RESET}
{G} /      /        o{RESET}
{G}/      /{RESET}
{G}\      \{RESET}
{G} \      \__________ o{RESET}
{G}  \               /{RESET}
{G}   \_____________/{RESET}"#
            }
            Distro::Debian => {
                r#"
{R}  _____{RESET}
{R} / {RESET} _{R}_ \{RESET}
{R}| {RESET} /{R}    |{RESET}
{R}{RESET}|  \{R}_{RESET}{R}__-{RESET}
{RESET}-{R}_
{RESET}  -{R}-_{RESET}"#
            }
            Distro::Unknown => {
                r#"


{R} _      ____  ____  ____  _____ _____ _____ ____ _{RESET}
{O}/ \  /|/  _ \/  _ \/  __\/    //  __//__ __Y   _Y \ /|{RESET}
{Y}| |\ ||| / \|| / \||  \/||  __\|  \    / \ |  / | |_||{RESET}
{G}| | \||| \_/|| \_/||    /| |   |  /_   | | |  \_| | ||{RESET}
{B}\_/  \|\____/\____/\_/\_\\_/   \____\  \_/ \____|_/ \|{RESET}
                                                      "#
            }
            Distro::EndeavourOS => {
                r#"

            {M}/o.{RESET}
         {O}/{RESET}{M}sssso{RESET}{B}-{RESET}
        {O}/{RESET}{M}ossssssso{RESET}{B}:{RESET}
     {O}/{RESET}{M}sssssssssss{RESET}{B}o+{RESET}
   {O}/{RESET}{M}ssssssssssssssso{RESET}{B}+{RESET}
  {O}//{RESET}{M}osssssssssssssso{RESET}{B}+-{RESET}
  {B} `+++++++++++++++-`{RESET}"#
            }
            Distro::Trisquel => {
                r#"                           ..
                        <<!OOOO~
                     !!mm`    ;;.
                    ac`        ;?
            ...     !c ..!   .;O`
      .. fffMMMMf`. `XX!aa  ..?`
     .DDDDDDDDfMMMMff   .  fCC
    CDD`     ..  fMMMf   fff`
  .C!.   !6666     MMM`fMf..
  .C .  !!   !h    MMMMMM`
  .L    !    .h     MMMM`    ....
  .??..      .^     MMf    66;;!JJJ
   `??? ..  .'^    .fRf  666      RR
    `?;;;;;;''     .fRf  6`        R.
       ...         .!Rf  !hh !!    ^.
                    !fCC   mm`     ^.
                     `CCC!      o;;^
                        CC``;;;!o
                           ...    "#
            }
            Distro::NixOS => {
                r#"  ▗▄   ▗▄ ▄▖
 ▄▄🬸█▄▄▄🬸█▛ ▃
   ▟▛    ▜▃▟🬕
🬋🬋🬫█      █🬛🬋🬋
 🬷▛🮃▙    ▟▛
 🮃 ▟█🬴▀▀▀█🬴▀▀
  ▝▀ ▀▘   ▀▘"#
            }
            Distro::Bazzite => {
                r#"
{M}    \\KK999999000009999999{RESET}
{M}   --\++++{RESET}KKKK{M}++++++++++++++,-{RESET}
{M}  \+++++++{RESET}KKKK{M}++++++++++==~```,,-{RESET}
{M}  9+++++++{RESET}KKKK{M}++++++++++~~````+++--{RESET}
{M}9{RESET}KKNNNNKKKKTKKKKKNNNNNNKKK{M}+`````+9{RESET}
{M}9{RESET}KKKKTTTTTTTTTTKTKKKDDDKKKK{M}``````0{RESET}
{M}9++{RESET}{M}+++++{RESET}KKLK{M}+++++++++++++{RESET}KKKK{M}````0{RESET}
{M}0++{RESET}{M}+++++{RESET}KKLK{M}++++{RESET}{B}++++++++++{RESET}TKN{M}````0{RESET}
{M}0{B}+++++++{RESET}KNLK{B}+++++++++++++++{RESET}KN{M}```{RESET}{M}`0{RESET}
{M}0{RESET}{B}++++++~{RESET}KNLK{RESET}{B}+++++++++++++++{RESET}KK{B}````{RESET}{M}9{RESET}
{M}0{RESET}{B}++++++~{RESET}KNLK{B}++++++++++++++{RESET}KKK{B}````{RESET}{M}9{RESET}
  9+++++++KKKK+++++++++++[KKKK+````9
  9+++++++KKKN+++++++++NNNNKKK`````9
  9++++++++KKN++++++[[[KNKKKK``````9
   vv+++++++KNKKNNNNKKKKKKK```````-
    vv++++==+KKKKKKKKKKK````````,,
      +vvv,vv+~~"""""""">M>~~`--
            v99999999999          "#
            }
            Distro::Manjaro => {
                r#"

{G}||||||||| ||||{RESET}
{G}||||||||| ||||{RESET}
{G}||||      ||||{RESET}
{G}|||| |||| ||||{RESET}
{G}|||| |||| ||||{RESET}
{G}|||| |||| ||||{RESET}
{G}|||| |||| ||||{RESET}
"#
            }
            Distro::Artix => {
                r#"{B}            '
{B}           'A'{RESET}
{B}          'ooo'{RESET}
{B}         'ookxo'{RESET}
{B}         `ookxxo'{RESET}
{B}       '.   `ooko'{RESET}
{B}      'ooo`.   `oo'{RESET}
{B}     'ooxxxoo`.   `'{RESET}
{B}    'ookxxxkooo.`   .{RESET}
{B}   'ookxxkoo'`   .'oo'{RESET}
{B}  'ooxoo'`     .:ooxxo'{RESET}
{B} 'io'`             `'oo'{RESET}
'`                     `'{RESET}"#
            }
            Distro::Void => {
                r#"
{G}    _______{RESET}
{G} _ \______ -{RESET}
{G}| \  ___  \ |{RESET}
{G}| | /   \ | |{RESET}
{G}| | \___/ | |{RESET}
{G}| \______ \_|{RESET}
{G} -_______\{RESET}"#
            }
            Distro::ALT => {
                r#"
    ``````````````````````````````````
    ``````````````````````````````````
    ``````````````````````````````````
           .    .    `````````````````
     ___   |   _/_   `````````````````
        `  |    |    `````````````````
   |    |  |    |    `````````````````
   `.__/| /\__  \__/ `````````````````
                     `````````````````
    ```  .           `````````````````
    ```  |   ` , __   ,   . _  .- ````
    ```  |   | |'  `. |   |  \,'  ````
    ```  |   | |    | |   |  /\   ````
    ``` /\__ / /    | `._/| /  \  ````
    ```                           ````
    ```                           ````"#
            }
            Distro::Guix => {
                r#"

  {Y}|.__          __.|{RESET}
  {Y}|__ \        / __|{RESET}
  {Y}   \ \      / /{RESET}
  {Y}    \ \    / /{RESET}
  {Y}     \ \  / /{RESET}
  {Y}      \ \/ /{RESET}
  {Y}       \__/{RESET}"#
            }
            Distro::Kali => {
                r#"
     -#. #
      @###
{B}  -######{RESET}
{B} @#########{RESET}
{B}=##. {RESET} .#####
{B}##     {RESET} ## ##
{B}##       {RESET}## #
{B}##       {RESET}@###
{B}##.        {RESET}###
{B}##%       {RESET}##-
{B}  -##%{RESET}    -*
{B}   :*##+{RESET}
{B}     :*#*{RESET}
{B}       -#{RESET}
{B}        @{RESET}
{B}        :{RESET}"#
            }
            Distro::OpenSUSE => {
                r#"

{G} .oooo.{RESET}
{G}o   o  o{RESET}
{G}ooooo  oo{RESET}
{G}o      oo{RESET}
{G} 'oooooooooooo.{RESET}
{G}       oo      o{RESET}
{G}       oo  ooooo{RESET}
{G}        o  o   o{RESET}
{G}         'oooo'{RESER}"#
            }
            Distro::Lubuntu => {
                r#"                 ...........
                `77MMMMMMMMMMM``
               hhMM``      `vMM======
              !hhTT         `d>===`
    MMh'      !hhT
    MMhhhh    !h?
    MM`  hhh    ?            I
    MMM    ffn`              Y'
     MM      ffn             Yj
      M                      jj
      Mn                      jW'
  MMMMMMnnnn`                  WW
  `MMMoooonnnnnn`              WW'
   `MM`     `ooonn             WMM
     MM`                        NN
      MMp`                     NN'
       MMpvv`               ooNN
          `vvvvv'          ggg'"#
            }
            Distro::Xubuntu => {
                r#"{B}             __ygg@@@@@@@@@ggy__                                   {RESET}
{B}         _yg@@@@@@@@@@@@@@@@@@@@@gy_                               {RESET}
{B}      _a@@@@@@@@@@@@@@@@@@@@@@@@@@@@@y_                            {RESET}
{B}    _a@@@@@@@@@@@@@@@@@@@@@@@{RESET}#{RESET}{B}@@@@@@@@@g_                          {RESET}
{B}   a@@@@@@@@@@@@@{RESET}###{RESET}{B}@@@@@@@@{RESET}##{RESET}{B}@@@@{RESET}##{RESET}{B}@@@@@k                         {RESET}
{B}  g@@@@@@@{RESET}###{RESET}{B}@@@{RESET}#####{RESET}{B}@@@@@@@{RESET}##{RESET}{B}@@{RESET}###{RESET}{B}@@@@@@@k                       {RESET}
{B} a@@@@@@@@{RESET}#####{RESET}{B}@{RESET}#####{RESET}{B}@@@@@@{RESET}##{RESET}{B}@@{RESET}###{RESET}{B}@@@@@@@@@k                     {RESET}
{B}j@@@@@@@@@{RESET}############{RESET}{B}@@@@@{RESET}##{RESET}{B}@{RESET}###{RESET}{B}@@@@@@@@@@@k                   {RESET}
{B}g@@@@@@@@@{RESET}#####################{RESET}{B}@@@@@@@@@@@@@@                  {RESET}
{B}@@@@@@@@@{RESET}##########################{RESET}{B}@@@@@@@@@@                 {RESET}
{B}0@@@@@@@@{RESET}###########################{RESET}{B}@@@@@@@@@                {RESET}
{B}~@@@@@@@{RESET}############################{RESET}{B}@@@@@@@@F               {RESET}
{B} 9@@@@@@{RESET}##########################{RESET}{B}@@@@@@@@@P              {RESET}
{B}  4@@@@@@{RESET}######################{RESET}{B}@@@@@@@@@@@P             {RESET}
{B}   ~@@@@@@{RESET}################{RESET}{B}@@@@@@@@@@@@@@@F            {RESET}
{B}    `4@@@@@@{RESET}#######{RESET}{B}@@@@@@@@@@@@@@@@@@@@P`           {RESET}
{B}      `~@@@@@@@@@@@@@@@@@@@@@@@@@@@@@F`                             {RESET}
{B}         ~~4@@@@@@@@@@@@@@@@@@@@@P~~                                {RESET}
"#
            }
            Distro::Vanilla => {
                r#"

{Y}      ,x.{RESET}
{Y}     ;&?^.{RESET}
{Y}.-e~^+7'  )adbx,{RESET}
{Y} \#\.  `,*~ ~*/{RESET}
{Y}  `~*+-'-<ay,^{RESET}
{Y}  ,/  ,%\ `\&,{RESET}
{Y}  !&UP*  +./%?{RESET}"#
            }
            Distro::Garuda => {
                r#"
     .----.
   .'   ,  '.
 .'    '-----|
'.   -----,
  '.____.'"#
            }
            Distro::Nobara => {
                r#"    ...           """""""""
  f*00000*     ((@@@@@@@@@@@))
 .0000000*! JJJMMMMMMMMMMMMMM@LLLL``
 .000000mm?JJJJMMMMMMMMMMMMMMMMMMMM@``
 .000000mm?JJJJ@@MMMMMMMMMMMMMMMMMMMMMt
 .000000mmTJXJJJJ```DDDDDDMMMMMMMMMMMMM
 .00000mmmmJXJJ`        0DDDMMMMMMMMMMM
 .00000mmmmJXJ`           0DJMMMMMMMMMM
 .00000mmJmJfJ   @@@        JJMMMMMMMMM
 .00000mmJmJf`    @@@@@@@@  `JMMMMMMMMM
 .00000mmJmJf   @@ @@@       aaaaa`MMMM
 .000000mJmJf   @@@@@@           ---MMM
 .000000mmmJf                 X``MMMMMM
 .0000000mmqff                XHHMMMMMM
 .0000000mmqqqff              XHHMMMMMM
 .0000000mm0qqqqq             XHHMMMMMM
 .0000000mm0qqqqqq            @HHMMMMMM
 .0000000mm` ..qqq            @@HMMMMMM
  f0000000u    .qq            ``@@@@@@@
    .000`       .q                `````
"#
            }
            Distro::Tails => {
                r#"      ``
  ./yhNh
syy/Nshh         `:o/
N:dsNshh  █   `ohNMMd
N-/+Nshh      `yMMMMd
N-yhMshh       yMMMMd
N-s:hshh  █    yMMMMd so//.
N-oyNsyh       yMMMMd d  Mms.
N:hohhhd:.     yMMMMd  syMMM+
Nsyh+-..+y+-   yMMMMd   :mMM+
+hy-      -ss/`yMMMM     `+d+
  :sy/.     ./yNMMMMm      ``
    .+ys- `:+hNMMMMMMy/`
      `hNmmMMMMMMMMMMMMdo.
       dMMMMMMMMMMMMMMMMMNh:
       +hMMMMMMMMMMMMMMMMMmy.
         -oNMMMMMMMMMMmy+.`
           `:yNMMMds/.`
              .//`"#
            }
            Distro::RedHat => {
                r#"
{R}      .M.:MMM{RESET}
{R}     MMMMMMMMMM.{RESET}
{R}    ,MMMMMMMMMMM{RESET}
{R} .MM MMMMMMMMMMM{RESET}
{R}MMMM   MMMMMMMMM{RESET}
{R}MMMMMM           MM{RESET}
{R} MMMMMMMMM     ,MMMM{RESET}
{R}   MMMMMMMMMMMMMMMM:{RESET}
{R}      `MMMMMMMMMMMM {RESET}"#
            }
            Distro::Calculate => {
                r#"                              ......
                           ,,+++++++,.
                         .,,,....,,,+**+,,.
                       ............,++++,,,
                      ...............
                    ......,,,........
                  .....+*#####+,,,*+.
              .....,*###############,..,,,,,,..
           ......,*#################*..,,,,,..,,,..
         .,,....*####################+***+,,,,...,++,
       .,,..,..*#####################*,
     ,+,.+*..*#######################.
   ,+,,+*+..,########################*
.,++++++.  ..+##**###################+
.....      ..+##***#################*.
           .,.*#*****##############*.
           ..,,*********#####****+.
     .,++*****+++*****************+++++,.
      ,++++++**+++++***********+++++++++,
     .,,,,++++,..  .,,,,,.....,+++,.,,"#
            }
            Distro::CentOS => {
                r#" ____^____
 |\  |  /|
 | \ | / |
<---- ---->
 | / | \ |
 |/__|__\|
     v"#
            }
            Distro::ElementaryOS => {
                r#"         eeeeeeeeeeeeeeeee
      eeeeeeeeeeeeeeeeeeeeeee
    eeeee  eeeeeeeeeeee   eeeee
  eeee   eeeee       eee     eeee
 eeee   eeee          eee     eeee
eee    eee            eee       eee
eee   eee            eee        eee
ee    eee           eeee       eeee
ee    eee         eeeee      eeeeee
ee    eee       eeeee      eeeee ee
eee   eeee   eeeeee      eeeee  eee
eee    eeeeeeeeee     eeeeee    eee
 eeeeeeeeeeeeeeeeeeeeeeee    eeeee
  eeeeeeee eeeeeeeeeeee      eeee
    eeeee                 eeeee
      eeeeeee         eeeeeee
         eeeeeeeeeeeeeeeee"#
            }
            Distro::PopOS => {
                r#"
{B}             /////////////{RESET}
{B}          /////////////////////{RESET}
{B}       ///////{RESET}767{B}////////////////{RESET}
{B}    //////{RESET}7676767676{B}*//////////////{RESET}
{B}   /////{RESET}76767{B}//{RESET}7676767{B}//////////////{RESET}
{B}  /////{RESET}767676{B}///*{RESET}76767{B}///////////////{RESET}
{B} ///////{RESET}767676{B}///{RESET}76767{B}.///{RESET}7676{B}*///////{RESET}
{B} /////////{RESET}767676{B}//{RESET}76767{B}///{RESET}767676{B}////////{RESET}
{B} //////////{RESET}76767676767{B}////{RESET}76767{B}/////////{RESET}
{B} ///////////{RESET}76767676{B}//////{RESET}7676{B}//////////{RESET}
{B} ////////////,{RESET}7676{B},///////{RESET}767{B}///////////{RESET}
{B} /////////////{B}*{RESET}7676{B}///////{RESET}76{B}////////////{RESET}
{B} ///////////////{RESET}7676{B}////////////////////{RESET}
{B} ///////////////{RESET}7676{B}///{RESET}767{B}////////////{RESET}
{B}  //////////////////////'////////////{RESET}
{B}   //////.{RESET}7676767676767676767{B},//////{RESET}
{B}     /////{RESET}767676767676767676767{B}/////{RESET}
{B}       ///////////////////////////{RESET}
{B}          /////////////////////{RESET}
{B}              /////////////{RESET}"#
            }
            Distro::Devuan => {
                r#"
{R}..:::.{RESET}
{R}    ..-==-{RESET}
{Y}        .+#:{RESET}
{Y}         =@@{RESET}
{M}      :+%@#:{RESET}
{R}.:=+#@@%*:{RESET}
{R}#@@@#=:{RESET}"#
            }
            Distro::Deepin => {
                r#"             ............
         .';;;;;.       .,;,.
      .,;;;;;;;.       ';;;;;;;.
    .;::::::::'     .,::;;,''''',.
   ,'.::::::::    .;;'.          ';
  ;'  'cccccc,   ,' :: '..        .:
 ,,    :ccccc.  ;: .c, '' :.       ,;
.l.     cllll' ., .lc  :; .l'       l.
.c       :lllc  ;cl:  .l' .ll.      :'
.l        'looc. .   ,o:  'oo'      c,
.o.         .:ool::coc'  .ooo'      o.
 ::            .....   .;dddo      ;c
  l:...            .';lddddo.     ,o
   lxxxxxdoolllodxxxxxxxxxc      :l
    ,dxxxxxxxxxxxxxxxxxxl.     'o,
      ,dkkkkkkkkkkkkko;.    .;o;
        .;okkkkkdl;.    .,cl:.
            .,:cccccccc:,."#
            }
            Distro::FreeBSD => {
                r#"


 {R}/\,-'''''-,/\{RESET}
 {R}\_)       (_/{RESET}
 {R}|           |{RESET}
 {R}|           |{RESET}
 {R} ;         ;{RESET}
 {R}  '-_____-'{RESET}"#
            }
            Distro::NetBSD => {
                r#"

 \\{O}`-______,----__{RESET}
  \\{O}'.        __,---`_{RESET}
   \\{O}'.       `.____{RESET}
    \\{O}'-______,----`-{RESET}
     \\
      \\
       \\
        \\"#
            }
            Distro::OpenBSD => {
                r#"

{Y}      _____{RESET}
{Y}    \-     -/{RESET}
{Y} \_/         \{RESET}
{Y} |        O O |{RESET}
{Y} |_  <   )  3 ){RESET}
{Y} / \         /{RESET}
{Y}    /-_____-\{RESET}"#
            }
            Distro::GNU => {
                r#"
    _-`````-,           ,- '- .
  .'   .- - |          | - -.  `.
 /.'  /                     `.   \
:/   :      _...   ..._      ``   :
::   :     /._ .`:'_.._\.    ||   :
::    `._ ./  ,`  :    \ . _.''   .
`:.      /   |  -.  \-. \\_      /
  \:._ _/  .'   .@)  \@) ` `\ ,.'
     _/,--'       .- .\,-.`--`.
       ,'/''     (( \ `  )
        /'/'  \    `-'  (
         '/''  `._,-----'
          ''/'    .,---'
           ''/'      ;:
             ''/''  ''/
               ''/''/''
                 '/'/'
                  `;"#
            }
            Distro::Tux => {
                r#"

  {Y}    .--.{RESET}
   {Y}  |{RESET}o_o{Y} |{RESET}
   {Y}  |:_/ |{RESET}
 {B}   /{   \ \{RESET}
 {B}  (|     | ){RESET}
{Y}  /'\_   _/`\{RESET}
{Y}  \___)=(___/{RESET}
"#
            }
            Distro::Slackware => {
                r#"
   ________
  /  ______|
  | |______
  \______  \
   ______| |
| |________/
|____________
"#
            }
            Distro::Zorin => {
                r#"
    {J}  ###########{RESET}
   {J}  #############{RESET}

 {J}  #######=    -=###{RESET}
{J}  ######=    -=######{RESET}
 {J}  ###=    -=#######{RESET}

   {J}  #############{RESET}
    {J}  ###########{RESET}"#
            }
            Distro::Alpine => {
                r#"
{J}         ,db,{RESET}
{J}       ,d{G}%%%b{J},  ,db,{RESET}
{B}     ,{G}%%%P'{G}%%%b{J},d{G}%%%b{J},{RESET}
{B}   ,{G}%%%P{J},   `{G}%%%b{J}'{O}^q{G}%%b{J},{RESET}
{B} ,{G}%%%P{J},d|     `{G}%%%b{J} '{O}^q{G}%%b{J},{RESET}
{J}`{G}%%%'{J} '{O}q${J}|       '{G}%%%b{J}`{O}q{G}%%b{RESET}"#
            }
            Distro::LinuxMint => {
                r#"
{G} __________{RESET}
{W}|_          \{RESET}
{W} | {G} _____ {W}|{RESET}
{W} | {G}| | | {W}|{RESET}
{W} | {G}| | | {W}|{RESET}
{W} | {G}\_____/ {W}|{RESET}
{G} \_________/{RESET}"#
            }
            Distro::Rocky => {
                r#"
{G}    _________{RESET}
{G}   /         \{RESET}
{G}  |  {O}~ ~ ~{G}  |{RESET}
{G}  |  {O}~ ~ ~{G}  |{RESET}
{G}   \_________/{RESET}
{G}    \       /{RESET}
{G}     \_____/{RESET}"#
            }
            Distro::Solus => {
                r#"
{M}      .---.{RESET}
{M}     /     \{RESET}
{B}    |  {G}S{M}  |{RESET}
{B}     \     /{RESET}
{M}      `---'{RESET}
{M}       | |{RESET}"#
            }
            Distro::SteamOS => {
                r#"
{J}              .,,,,.{RESET}
{J}        .,'onNMMMMMNNnn',.{RESET}
{J}     .'oNMANKMMMMMMMMMMMNNn'.{RESET}
{B}   .'ANMMMMMMMXKNNWWWPFFWNNMNn'.{RESET}
{B}  ;NNMMMMMMMMMMNWW'' ,.., 'WMMM,{RESET}
{B} ;NMMMMV+##+VNWWW' .+;'':+, 'WMW,{RESET}
{B},VNNWP+{J}######{B}+WW,  {J}+:   +MMM,{RESET}
{B}'{J}#############{B},   +.    ,+'{J}+NM.{RESET}
{J}  '*#########*'     '*,,*' {B}+NMM#;{RESET}
{J}     `'*###*'          ,.,;###{B}+W.,{RESET}
{J}         .,;;,      .;##########{B};{RESET}
{J},',.         ';  ,+##############'{RESET}
{J} '###+. :,. .,; ,###############'{RESET}
{J}  '####.. `'' .,###############'{RESET}
{J}    '#####+++################'{RESET}
{J}      '*##################*'{RESET}
{J}         ''*##########*''{RESET}
{J}              ''''''{RESET}"#
            }
            Distro::KdeNeon => {
                r#"
{M}    __  __{RESET}
{M}   /  \/  \{RESET}
{B}  | {G}KDE{M} |{RESET}
{B}   \    /{RESET}
{M}    \  /{RESET}
{M}     \/ {RESET}"#
            }
            Distro::Parrot => {
                r#"
{G}    ,___{RESET}
{G}   /{Y}v v{G}\{RESET}
{G}  |{Y} > <{G} |{RESET}
{G}   \{Y}___/{G}/{RESET}
{G}    {Y}|||{RESET}
{G}   /   \{RESET}"#
            }
            Distro::ArcoLinux => {
                r#"
{O}        /\{RESET}
{O}       /  \{RESET}
{B}      / {O}A{B}  \{RESET}
{B}     /    \{RESET}
{O}    /______\{RESET}
{O}   /        \{RESET}"#
            }
            Distro::Asahi => {
                r#"
{G}       .:{RESET}
{G}      /  \{RESET}
{G}     / {R}@{G}  \{RESET}
{G}    |      |{RESET}
{G}     \    /{RESET}
{G}      \  /{RESET}
{G}       \/ {RESET}"#
            }
            Distro::MXLinux => {
                r#"
{B}  __  __{RESET}
{B} |  \/  |{RESET}
{G} | {B}MX{G} |{RESET}
{B} |      |{RESET}
{B}  \____/{RESET}"#
            }
        };
        Self::apply_color_tokens(art)
    }
    
    fn apply_color_tokens(s: &str) -> String {
    const TOKENS: &[(&str, &str)] = &[
        ("{G}", "\x1b[32m"),
        ("{J}", "\x1b[36m"),
        ("{Y}", "\x1b[33m"),
        ("{O}", "\x1b[38;5;208m"),
        ("{R}", "\x1b[31m"),
        ("{M}", "\x1b[35m"),
        ("{B}", "\x1b[34m"),
        ("{W}", "\x1b[97m"),
        ("{RESET}", "\x1b[0m"),
    ];

    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let matched = TOKENS.iter().find(|(tok, _)| s[i..].starts_with(tok));
            if let Some((tok, ansi)) = matched {
                out.push_str(ansi);
                i += tok.len();
                continue;
            }
        }

        let ch = s[i..].chars().next().unwrap();
        let len = ch.len_utf8();
        out.push(ch);
        i += len;
    }

    out
    }
}

#[cfg(test)]
mod tests {
    use super::Distro;

    #[test]
    fn detects_new_distros() {
        assert_eq!(Distro::from_string("alpine"), Distro::Alpine);
        assert_eq!(Distro::from_string("Alpine Linux"), Distro::Alpine);
        assert_eq!(Distro::from_string("linuxmint"), Distro::LinuxMint);
        assert_eq!(Distro::from_string("LinuxMint"), Distro::LinuxMint);
        assert_eq!(Distro::from_string("Linux Mint 22"), Distro::LinuxMint);
        assert_eq!(Distro::from_string("Rocky Linux 9"), Distro::Rocky);
        assert_eq!(Distro::from_string("SteamOS"), Distro::SteamOS);
        assert_eq!(Distro::from_string("KDE neon"), Distro::KdeNeon);
        assert_eq!(Distro::from_string("ArcoLinux"), Distro::ArcoLinux);
        assert_eq!(Distro::from_string("devuan"), Distro::Devuan);
        assert_ne!(Distro::from_string("alpine"), Distro::GNU);
        assert_eq!(Distro::from_string("oracle"), Distro::Unknown);
        assert_eq!(Distro::from_string("omarchy"), Distro::Unknown);
    }
}
