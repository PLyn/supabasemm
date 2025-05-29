use leptos::prelude::*;

#[component]
pub fn AnimatedArrow() -> impl IntoView {
    view!(
        <svg width="80" height="120" viewBox="0 0 80 120" xmlns="http://www.w3.org/2000/svg">
        <defs>
            <linearGradient id="arrowGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
            </linearGradient>

            <filter id="glow">
            <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
            <feMerge>
                <feMergeNode in="coloredBlur"/>
                <feMergeNode in="SourceGraphic"/>
            </feMerge>
            </filter>
        </defs>

        <path d="M35 10 L35 65 L20 65 L40 90 L60 65 L45 65 L45 10 Z"
                fill="url(#arrowGradient)"
                filter="url(#glow)">
            <animateTransform attributeName="transform"
                            type="translate"
                            values="0,0; 0,3; 0,0"
                            dur="2s"
                            repeatCount="indefinite"/>
        </path>

        <g opacity="0.7">
            <path d="M38 18 L38 21 L36 21 L40 25 L44 21 L42 21 L42 18 Z" fill="#667eea">
            <animate attributeName="opacity"
                    values="0.8;0.6;0.4;0.2;0.1;0;0;0;0.8"
                    dur="3s"
                    repeatCount="indefinite"/>
            <animateTransform attributeName="transform"
                                type="translate"
                                values="0,0; 0,8; 0,16; 0,24; 0,32; 0,40; 0,48; 0,0; 0,0"
                                dur="3s"
                                repeatCount="indefinite"/>
            </path>
        </g>

        <g opacity="0.5">
            <path d="M37 23 L37 26 L35 26 L40 30 L45 26 L43 26 L43 23 Z" fill="#764ba2">
            <animate attributeName="opacity"
                    values="0.6;0.4;0.3;0.2;0.1;0;0;0;0.6"
                    dur="3s"
                    begin="0.5s"
                    repeatCount="indefinite"/>
            <animateTransform attributeName="transform"
                                type="translate"
                                values="0,0; 0,8; 0,16; 0,24; 0,32; 0,40; 0,48; 0,0; 0,0"
                                dur="3s"
                                begin="0.5s"
                                repeatCount="indefinite"/>
            </path>
        </g>

        <g opacity="0.4">
            <path d="M38 28 L38 31 L36 31 L40 35 L44 31 L42 31 L42 28 Z" fill="#667eea">
            <animate attributeName="opacity"
                    values="0.4;0.3;0.2;0.1;0;0;0;0;0.4"
                    dur="3s"
                    begin="1s"
                    repeatCount="indefinite"/>
            <animateTransform attributeName="transform"
                                type="translate"
                                values="0,0; 0,8; 0,16; 0,24; 0,32; 0,40; 0,48; 0,0; 0,0"
                                dur="3s"
                                begin="1s"
                                repeatCount="indefinite"/>
            </path>
        </g>
        </svg>
    )
}
