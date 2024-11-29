export function colorNameToAnsiCode(colorName, background = false) {
  let color = {
    black: 30,
    red: 31,
    green: 32,
    yellow: 33,
    blue: 34,
    magenta: 35,
    cyan: 36,
    white: 37,
    brightblack: 90,
    gray: 90,
    grey: 90,
    brightred: 91,
    brightgreen: 92,
    brightyellow: 93,
    brightblue: 94,
    brightmagenta: 95,
    brightcyan: 96,
    brightwhite: 97,
  }[String(colorName ?? "").trim().toLowerCase()] ?? 0;
  if (color > 0 && background) color += 10;
  return color;
}

export function modNameToAnsiCode(modName) {
  return {
    reset: 0,
    normal: 0,
    bold: 1,
    dim: 2,
    italic: 3,
    underline: 4,
  }[String(modName ?? "").trim().toLowerCase()] ?? 0;
}

export function ansi(string, ...ansiCodes) {
  const effective = ansiCodes.filter(c => (
    c > 0 &&
    (
      c <= 4 ||
      (c >= 30 && c <= 37) ||
      (c => 40 && c <= 47) ||
      (c >= 90 && c <= 97) ||
      (c >= 100 && c <= 107)
    )
  ));
  if (effective.length) {
    return `\u001b[${effective.join(';')}m${String(string).replace(/(\u001b\[0m)+$/, '')}\u001b[0m`;
  }
  else return String(string);
}

export const c = (string, color, bg = false) => ansi(string, colorNameToAnsiCode(color, bg));
export const m = (string, mod) => ansi(string, modNameToAnsiCode(mod));

// All of the other exports wrap the string with a reset, so should never need this... but just in case.
export const reset = (string) => ansi(string);

export const bold = (string) => m(string, 'bold');
export const dim = (string) => m(string, 'dim');
export const italic = (string) => m(string, 'italic');
export const underline = (string) => m(string, 'underline');

export const black = (string) => c(string, 'black');
export const blackBg = (string) => c(string, 'black', true);
export const red = (string) => c(string, 'red');
export const redBg = (string) => c(string, 'red', true);
export const green = (string) => c(string, 'green');
export const greenBg = (string) => c(string, 'green', true);
export const yellow = (string) => c(string, 'yellow');
export const yellowBg = (string) => c(string, 'yellow', true);
export const blue = (string) => c(string, 'blue');
export const blueBg = (string) => c(string, 'blue', true);
export const magenta = (string) => c(string, 'magenta');
export const magentaBg = (string) => c(string, 'magenta', true);
export const cyan = (string) => c(string, 'cyan');
export const cyanBg = (string) => c(string, 'cyan', true);
export const white = (string) => c(string, 'white');
export const whiteBg = (string) => c(string, 'white', true);
export const brightBlack = (string) => c(string, 'brightBlack');
export const brightBlackBg = (string) => c(string, 'brightBlack', true);
export const gray = (string) => c(string, 'gray');
export const grayBg = (string) => c(string, 'gray', true);
export const grey = (string) => c(string, 'grey');
export const greyBg = (string) => c(string, 'grey', true);
export const brightRed = (string) => c(string, 'brightRed');
export const brightRedBg = (string) => c(string, 'brightRed', true);
export const brightGreen = (string) => c(string, 'brightGreen');
export const brightGreenBg = (string) => c(string, 'brightGreen', true);
export const brightYellow = (string) => c(string, 'brightYellow');
export const brightYellowBg = (string) => c(string, 'brightYellow', true);
export const brightBlue = (string) => c(string, 'brightBlue');
export const brightBlueBg = (string) => c(string, 'brightBlue', true);
export const brightMagenta = (string) => c(string, 'brightMagenta');
export const brightMagentaBg = (string) => c(string, 'brightMagenta', true);
export const brightCyan = (string) => c(string, 'brightCyan');
export const brightCyanBg = (string) => c(string, 'brightCyan', true);
export const brightWhite = (string) => c(string, 'brightWhite');
export const brightWhiteBg = (string) => c(string, 'brightWhite', true);