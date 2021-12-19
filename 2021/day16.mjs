function readPacket(bits, offset, stopOffset = 0) {
  if (offset + 6 >= bits.length) return; // Past the end of the data

  let ptr = offset;

  const version = parseInt(bits.substring(ptr, ptr + 3), 2);
  const type = parseInt(bits.substring(ptr + 3, ptr + 6), 2);
  ptr += 6;
  
  const packet = { version, type };

  // The simple case.
  if (type === 4) {
    let parts = [];
    while (true) {
      const final = bits.substring(ptr, ptr + 1) === '0';
      parts.push(bits.substring(ptr + 1, ptr + 5));
      ptr += 5;
      if (final || parts[parts.length - 1] == null) break; // Ran past end of data...
    }
    packet.literal = parseInt(parts.join(''), 2);
    return { offset: ptr, packet };
  }

  // For every other type, we have a preamble telling us how much data to expect
  packet.children = [];

  const lenType = parseInt(bits.substring(ptr, ptr + 1), 2);
  ptr++;

  if (lenType === 0) {
    if (ptr + 15 >= bits.length || (stopOffset > 0 && ptr + 15 >= stopOffset)) return null; // Ran past end of data
    // Next 15 bits describe the total length in bits of this packet's entire subtree
    packet.expectSize = parseInt(bits.substring(ptr, ptr + 15), 2);
    ptr += 15;
  }
  else if (lenType === 1) {
    if (ptr + 11 >= bits.length || (stopOffset > 0 && ptr + 11 >= stopOffset)) return null; // Ran past end of data
    // Next 11 bits describe the number of direct child packets in this packet
    packet.expectChildren = parseInt(bits.substring(ptr, ptr + 11), 2);
    ptr += 11;
  }
  else throw new Error(`Got lenType=${lenType}, should be 0 or 1. Pointer likely moved incorrectly.`);

  if (packet.expectChildren) {
    for (let c = 0; c < packet.expectChildren; c++) {
      const next = readPacket(bits, ptr, stopOffset);
      ptr = next.offset;
      packet.children.push(next.packet);
    }
  } else if (packet.expectSize) {
    const nextStop = ptr + packet.expectSize;
    while (ptr < nextStop) {
      const next = readPacket(bits, ptr, nextStop);
      ptr = next.offset;
      packet.children.push(next.packet);
    }
    if (ptr !== nextStop) throw new Error(`Expected to read from ${offset} until ${nextStop}, but read up to ${ptr}. ` + JSON.stringify({ type, version, packet }));
  }

  if (stopOffset > 0 && ptr > stopOffset) throw new Error(`Parser went out of bounds! Reached ${ptr}, but should have stopped at ${stopOffset}.`);

  return { offset: ptr, packet };
}

function sumVersionRecursive(packet) {
  return packet.version + (packet.children || []).reduce((a, c) => a + sumVersionRecursive(c), 0);
}

function operate(packet) {
  switch (packet.type) {
    case 0: // Sum all children's values
      return packet.children.reduce((a, c) => a + operate(c), 0);
    case 1: // Multiply all children's values
      return packet.children.reduce((a, c) => a * operate(c), 1);
    case 2: // Minimum value among children's values
      return Math.min(...packet.children.map(c => operate(c)));
    case 3: // Maximum value among children's values
      return Math.max(...packet.children.map(c => operate(c)));
    case 4: // Literal value. This will be the leaf nodes.
      return packet.literal;
    case 5: // 1 if first child is greater than second; 0 otherwise. MUST have exactly two children.
      if (packet.children.length !== 2) throw new Error('Packet type 5 expected two children, has ' + packet.children.length);
      return Number(operate(packet.children[0]) > operate(packet.children[1]));
    case 6: // 1 if first child is less than second; 0 otherwise. MUST have exactly two children.
      if (packet.children.length !== 2) throw new Error('Packet type 6 expected two children, has ' + packet.children.length);
      return Number(operate(packet.children[0]) < operate(packet.children[1]));
    case 7: // 1 if children have the same value; 0 otherwise. MUST have exactly two children.
      if (packet.children.length !== 2) throw new Error('Packet type 5  7 expected two children, has ' + packet.children.length);
      return Number(operate(packet.children[0]) === operate(packet.children[1]));
    default:
      throw new Error(`Invalid type: ${packet.type}`);
  }
}

// For debugging. Leaving it in for future readers that have no idea what's going on (like my future self).
function toFormula(packet, long = true, indent = 0) {
  const pad = long ? '  '.repeat(indent) : '';
  if (packet.type === 4) return pad + String(packet.literal);
  const op = ['+', '*', 'min', 'max', 'literal', '>', '<', '='][packet.type];
  return `${pad}${op}(${long ? '\n' : ''}${packet.children.map(p => toFormula(p, long, indent + 1)).join(long ? ',\n' : ', ')}${long ? '\n' : ''}${pad})`;
}

(await import('./aoc.mjs')).default(
  2021, 16,
  (bits) => {
    let ptr = 0;
    let packets = [];
    while (ptr < bits.length) {
      const next = readPacket(bits, ptr);
      if (!next) break;
      ptr = next.offset;
      packets.push(next.packet);
    }
    return packets.reduce((a, c) => a + sumVersionRecursive(c), 0);
  }, 20,
  (bits) => {
    let ptr = 0;
    let packets = [];
    while (ptr < bits.length) {
      const next = readPacket(bits, ptr);
      if (!next) break;
      ptr = next.offset;
      packets.push(next.packet);
    }
    if (packets.length !== 1) {
      throw new Error('Part 2 should have exactly one outer packet, but it has ' + packets.length);
    }
    return operate(packets[0]);
  }, 1,
  data => data[0].split('').map(c => parseInt(c, 16).toString(2).padStart(4, '0')).join('')
);