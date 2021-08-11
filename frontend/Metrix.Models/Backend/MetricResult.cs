using System;

namespace Metrix.Models
{
    public class MetricResult
    {
        public Guid Id { get; set; }
        public string DataPoint { get; set; }
        public string DataGroup { get; set; }
        public string DataType { get; set; }
        public double DataValueNumeric { get; set; }
        public DateTime CreatedDateTime { get; set; }
    }
}