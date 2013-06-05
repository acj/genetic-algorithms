import scala.util.Random
import ga.Chromosome

package object Defs {
  type EvaluatedChromosomes = List[(Chromosome, Float)]
  type FitnessFunction = Chromosome => Float;
  type Population = List[Chromosome]	
	
  def weightedRandomSelection(list: EvaluatedChromosomes): Chromosome = {
    def weightedRandomSelectionAcc(randNum: Float, list: EvaluatedChromosomes, totalProb: Float): Chromosome =
      if (randNum <= totalProb) list.head._1
      else weightedRandomSelectionAcc(randNum, list.tail, totalProb + list.head._2)

    val totalProb = list.collect{ case (_, weight) => weight }.sum
    weightedRandomSelectionAcc(Random.nextFloat() * totalProb, list, totalProb)
  }
}