use bitvec::prelude::BitVec;
use helpers::input_lines;
use itertools::Itertools;
use std::convert::TryFrom;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct BITSMessage {
    bits: BitVec,
}

impl FromStr for BITSMessage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .bytes()
            .map(|byte| {
                let value_result = match byte {
                    b if (b'0'..=b'9').contains(&b) => Ok(b - b'0'),
                    b if (b'A'..=b'F').contains(&b) => Ok(b - b'A' + 10),
                    _ => Err(anyhow::anyhow!(
                        "Input contain not valid hex characters: {value}",
                        value = byte
                    )),
                };
                value_result.map(|value| {
                    [
                        value & (1 << 3) != 0,
                        value & (1 << 2) != 0,
                        value & (1 << 1) != 0,
                        value & (1 << 0) != 0,
                    ]
                })
            })
            .flatten_ok()
            .collect::<Result<_, _>>()?;

        Ok(Self { bits })
    }
}

struct BITSMessageIter<'m> {
    msg: &'m BITSMessage,
    bit_to_read: usize,
}

impl Iterator for BITSMessageIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_to_read >= self.msg.bits.len() {
            None
        } else {
            let value = self.msg.bits[self.bit_to_read];
            self.bit_to_read += 1;
            Some(value)
        }
    }
}

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Min),
            3 => Ok(Self::Max),
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => Err(anyhow::anyhow!(
                "Unsupported operation: {value}",
                value = value
            )),
        }
    }
}

impl From<&Operation> for u8 {
    fn from(value: &Operation) -> Self {
        match value {
            Operation::Sum => 0,
            Operation::Product => 1,
            Operation::Min => 2,
            Operation::Max => 3,
            Operation::GreaterThan => 5,
            Operation::LessThan => 6,
            Operation::EqualTo => 7,
        }
    }
}
#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: usize,
    },
    SubPacketLength {
        version: u8,
        _bits_count: u16,
        operation: Operation,
        packets: Vec<Packet>,
    },
    SubPacketCount {
        version: u8,
        _packets_count: u16,
        operation: Operation,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn read_literal<I: Iterator<Item = bool>>(
        bits_message_iter: &mut I,
        version: u8,
    ) -> Result<Packet, anyhow::Error> {
        let mut value = 0;

        loop {
            let bits: BitVec = bits_message_iter.take(5).collect();
            anyhow::ensure!(
                bits.len() == 5,
                "Expected to read 5 more bits, but only {actual_length} bits were available",
                actual_length = bits.len()
            );

            value = bits[1..]
                .iter()
                .fold(value, |res, bit| (res << 1) + usize::from(*bit));
            if !bits[0] {
                return Ok(Packet::Literal { version, value });
            }
        }
    }

    fn read_subpacket_length<I: Iterator<Item = bool>>(
        bits_message_iter: &mut I,
        version: u8,
        operation: Operation,
    ) -> Result<Packet, anyhow::Error> {
        let bits_count = bits_message_iter
            .take(15)
            .fold(0, |res, bit| (res << 1) + u16::from(bit));

        let subpacket_bits: BitVec = bits_message_iter.take(bits_count as usize).collect();
        anyhow::ensure!(
            subpacket_bits.len() == bits_count as usize,
            "Expected to read {bits_count} more bits, but only {actual_length} bits were available",
            bits_count = subpacket_bits.len(),
            actual_length = subpacket_bits.len()
        );

        let mut packets = vec![];
        let mut peekable_iter = subpacket_bits.iter().by_vals().peekable();
        while peekable_iter.peek().is_some() {
            packets.push(Packet::read(&mut peekable_iter)?);
        }

        Ok(Packet::SubPacketLength {
            _bits_count: bits_count,
            operation,
            packets,
            version,
        })
    }

    fn read_subpacket_count<I: Iterator<Item = bool>>(
        bits_message_iter: &mut I,
        version: u8,
        operation: Operation,
    ) -> Result<Packet, anyhow::Error> {
        let packets_count = bits_message_iter
            .take(11)
            .fold(0, |res, bit| (res << 1) + u16::from(bit));

        let packets: Vec<_> = (0..packets_count)
            .map(|_| Packet::read(bits_message_iter))
            .collect::<Result<_, _>>()?;

        Ok(Packet::SubPacketCount {
            _packets_count: packets_count,
            operation,
            packets,
            version,
        })
    }

    fn read<I: Iterator<Item = bool>>(bits_message_iter: &mut I) -> Result<Self, anyhow::Error> {
        let version = bits_message_iter
            .take(3)
            .fold(0, |res, bit| (res << 1) + u8::from(bit));
        let packet_id = bits_message_iter
            .take(3)
            .fold(0, |res, bit| (res << 1) + u8::from(bit));

        if packet_id == 4 {
            Packet::read_literal(bits_message_iter, version)
        } else {
            let length_type_id = bits_message_iter.next().map_or(0, u8::from);
            match length_type_id {
                0 => Packet::read_subpacket_length(
                    bits_message_iter,
                    version,
                    Operation::try_from(packet_id)?,
                ),
                1 => Packet::read_subpacket_count(
                    bits_message_iter,
                    version,
                    Operation::try_from(packet_id)?,
                ),
                _ => Err(anyhow::anyhow!(
                    "length type ID is expected to be 0 or 1, received: {length_type_id}",
                    length_type_id = length_type_id
                )),
            }
        }
    }

    fn sum_versions(&self) -> usize {
        match self {
            Self::Literal { version, .. } => *version as usize,
            Self::SubPacketLength {
                version, packets, ..
            }
            | Self::SubPacketCount {
                version, packets, ..
            } => (*version as usize) + packets.iter().map(Packet::sum_versions).sum::<usize>(),
        }
    }

    fn evaluate(&self) -> usize {
        match self {
            Self::Literal { value, .. } => *value,
            Self::SubPacketLength {
                packets, operation, ..
            }
            | Self::SubPacketCount {
                packets, operation, ..
            } => {
                let packets_values = packets.iter().map(Packet::evaluate);

                match operation {
                    Operation::Sum => packets_values.sum(),
                    Operation::Product => packets_values.product(),
                    Operation::Min => packets_values.min().unwrap_or(usize::MAX),
                    Operation::Max => packets_values.max().unwrap_or(usize::MIN),
                    Operation::GreaterThan => {
                        let values: Vec<_> = packets_values.take(2).collect();
                        assert_eq!(values.len(), 2);
                        usize::from(values[0] > values[1])
                    }
                    Operation::LessThan => {
                        let values: Vec<_> = packets_values.take(2).collect();
                        assert_eq!(values.len(), 2);
                        usize::from(values[0] < values[1])
                    }
                    Operation::EqualTo => {
                        let values: Vec<_> = packets_values.take(2).collect();
                        assert_eq!(values.len(), 2);
                        usize::from(values[0] == values[1])
                    }
                }
            }
        }
    }
}

impl<'m> IntoIterator for &'m BITSMessage {
    type Item = bool;
    type IntoIter = BITSMessageIter<'m>;
    fn into_iter(self) -> Self::IntoIter {
        BITSMessageIter {
            msg: self,
            bit_to_read: 0,
        }
    }
}

fn part01(packet: &Packet) -> usize {
    packet.sum_versions()
}

fn part02(packet: &Packet) -> usize {
    packet.evaluate()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    anyhow::ensure!(lines.len() == 1);
    let bits_message: BITSMessage = lines[0].parse()?;
    let packet = Packet::read(&mut bits_message.into_iter())?;

    println!("Part 1: {}", part01(&packet));
    println!("Part 2: {}", part02(&packet));

    Ok(())
}
