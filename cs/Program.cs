using System.IO;

namespace cs;

class Program
{
    static byte[] Encode(byte[] inData)
    {
        var originalSize = (uint)inData.Length;

        var outArray = LZ4.LZ4Codec.Encode(inData, 0, inData.Length);

        var packedSize = (uint)outArray.Length + 4;

        var original = BitConverter.GetBytes(originalSize);
        var packed = BitConverter.GetBytes(packedSize);
        var zippedBytes = packed.Concat(original).Concat(outArray);

        return zippedBytes.ToArray();
    }

    static void Main(string[] args)
    {
        var bytes = File.ReadAllBytes(args[0]);
        File.WriteAllBytes(args[1], Encode(bytes));
    }
}
